import type { Identity } from "@clockworklabs/spacetimedb-sdk";
import { Checkbox, DbConnection, User } from '../module_bindings';

const AUTH_TOKEN_KEY = "auth_token"
const QUERIES = [
    "SELECT * FROM checkbox",
    "SELECT * FROM user",
]


class Store {
    startConnection() {
        this.connection = DbConnection.builder()
        .withUri("ws://localhost:3000")
        .withModuleName("one-million-checkboxes")
        .withToken(localStorage.getItem(AUTH_TOKEN_KEY) || "")
        .onConnect((conn, identity, token) => {
            this.identity = identity;
            this.connected = true;
            localStorage.setItem(AUTH_TOKEN_KEY, token);
            console.log("Connected to SpacetimeDB with identity:", identity.toHexString());

            // Set up reducers
            if (this.connection) {
                this.connection.reducers.onToggle(() => {
                    console.log("Toggle has occurred.");
                });
            }

            let count = 0;

            QUERIES.forEach(query => {
                conn.subscriptionBuilder().onApplied(() => {
                    if (++count === QUERIES.length) {
                        console.log("SDK client cache initialized.");
                    }
                }).subscribe(query);
            });
        })
        .onDisconnect(() => {
            console.log("Disconnected from SpacetimeDB");
            this.connected = false;
        })
        .onConnectError(error => {
            console.log("Error connecting to SpacetimeDB:", error);
        })
        .build();

        this.connection.db.checkbox.onInsert((ctx, checkbox) => {
            this.checkboxes[checkbox.id] = true
        })
    
        this.connection.db.checkbox.onUpdate((ctx, oldRow, newRow) => {
            this.checkboxes[oldRow.id] = false
            this.checkboxes[newRow.id] = true
        })
    
        this.connection.db.checkbox.onDelete((ctx, checkbox) => {
            this.checkboxes[checkbox.id] = false
        })
        
        this.connection.db.user.onInsert((ctx, user) => {
            this.users.push(user)
        })
    
        this.connection.db.user.onUpdate((ctx, oldUser, newUser) => {
            const index = this.users.findIndex((item) => item.identity === oldUser.identity);
            if (index === -1) return

            this.users[index] = { ...oldUser, ...newUser }
            
        })
    
        this.connection.db.user.onDelete((ctx, user) => {
            const index = this.users.findIndex((item) => item.identity === user.identity);
            if (index === -1) return

            this.users.splice(index, 1)
            
        })

        this.connection.db.checkbox.count()

    }

    connected = $state<boolean>(false)
    identity = $state<Identity>()
    connection = $state<DbConnection>()
    users = $state<User[]>([]);
    checkboxes = $state(new Array<boolean>(100_000))

}

export const store = new Store()