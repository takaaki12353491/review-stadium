import Header from "./index";
import Provider from "@/app/Provider";

export default {
  title: "Header",
  component: Header,
};

export const Unauthenticated = () => {
  return (
    <Provider>
      <Header />
    </Provider>
  );
};
