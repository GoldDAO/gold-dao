const errorMap = {
  NeuronHotKeyInvalid: 'Your principal is not a hotkey on this neuron. Please add your principal as the hotkey in the NNS app. https://docs.gold-dao.org/how-to/receive-rewards/configure-neurons',
  NeuronDoesNotExist: 'A Neuron with ID does not exist! Are you sure you copied the correct Neuron ID?',
  NeuronHotKeyAbsent: 'This Neuron does not contain any hotkeys. Please add your principal as the hotkey in the NNS app. https://docs.gold-dao.org/how-to/receive-rewards/configure-neurons',
  NeuronOwnerInvalid: 'A different user (principal) has their hotkey on this Neuron. If you added your hotkey under a different wallet or principal you must first login with that user, remove the neuron and then remove the hotkey from the neuron in the NNS in this order.',
  InternalError: 'An internal error occurred. Please report this on the telegram chat channel.',
  NeuronNotClaimed: 'This Neuron has not been added via this dashboard.',
  TokenSymbolInvalid: "Can't claim this token because it isn't a valid reward token.",
  TransferFailed: 'The transfer failed. Please try again and report this in the telegram chat channel.',
};

const mapResponseErrorCodeToFriendlyError = (errorResponse) => {
  const keys = Object.keys(errorResponse);
  const defaultErrorMessage = 'Something went wrong.';

  if (keys.length === 0) {
    return defaultErrorMessage;
  }
  const key = keys[0];

  const niceError = errorMap[key];
  if (niceError !== undefined) {
    return niceError;
  }
  return defaultErrorMessage;
};

export default mapResponseErrorCodeToFriendlyError;
