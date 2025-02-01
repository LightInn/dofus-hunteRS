import {Link} from "react-router-dom";


export default function Navbar() {


    return (
        <div>
            <nav>
                <ul style={{display: 'flex', gap: '1rem'}}>
                    <li>
                        <Link to="/">Home</Link>
                    </li>
                    <li>
                        <Link to="/debug">Debug</Link>
                    </li>
                    <li>
                        <Link to="/settings">Settings</Link>
                    </li>
                </ul>
            </nav>
        </div>
    )
}