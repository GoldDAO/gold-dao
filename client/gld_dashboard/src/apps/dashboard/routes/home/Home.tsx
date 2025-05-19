import GOLDAOTokenOverview from "./-components/GOLDAOTokenOverview";
import MainChart from "./-components/MainChart";
import GLDTTokenOverview from "./-components/GLDTTokenOverview";
import GoldDAOOwnedNeuronsOverview from "./-components/GoldDAOOwnedNeuronsOverview";
import BuyBackAndBurnOverview from "./-components/buy_back_and_burn_overview/BuyBackAndBurnOverview";
import Dexs from "./-components/Dexs";

const Home = () => {
  return (
    <div className="py-4 xl:py-8">
      <div className="mb-8">
        <div className="text-4xl font-bold text-accent">Gold DAO</div>
        <div className="text-4xl">Overview</div>
      </div>
      <GOLDAOTokenOverview className="mb-4" />
      <MainChart className="mb-4" />
      <GLDTTokenOverview className="mb-4" />
      <GoldDAOOwnedNeuronsOverview className="mb-4" />
      <BuyBackAndBurnOverview className="mb-4" />
      <Dexs className="mb-4" />
    </div>
  );
};

export default Home;
