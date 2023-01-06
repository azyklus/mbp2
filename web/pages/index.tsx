import { Main } from '../components/layouts/Main';
import { ReactDOMClient, createRoot } from 'react-dom';

export default function Home() {
   return <Main />;
}

const root = createRoot(<Home />);
root.render(document.getElementById("app"))
