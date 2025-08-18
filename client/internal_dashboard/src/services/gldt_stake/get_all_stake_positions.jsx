const get_all_stake_positions = async (actor) => {
  const result = await actor.get_all_stake_positions([]);
  return result;
};

export default get_all_stake_positions;
