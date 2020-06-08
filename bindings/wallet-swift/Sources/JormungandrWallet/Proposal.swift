import Foundation
import JormungandrWalletC

class Proposal {
    internal var pointer: ProposalPtr

    init(votePlanId: Data, payloadType: VotePayloadType, index: UInt8, numChoices: UInt8) throws {
        self.pointer = try WalletC.Proposal.new(
            votePlanId: votePlanId,
            payloadType: payloadType,
            index: index,
            numChoices: numChoices
        )
    }

    deinit {
        WalletC.Proposal.delete(proposal: self.pointer)
    }
}
