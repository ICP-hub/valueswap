import LandingPage from "./pages/LandingPage";
import HomePage from "./pages/HomePage.jsx";

const AppRoutes = [

    {
        path: "/",
        page: <LandingPage />,
    },
    {
        path: "/valueswap",
        page: <HomePage />,
    },
    {
        path: "/valueswap/*",
        page: <HomePage />,
    },

]


export default AppRoutes;