use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Rucksack {
    pub items: String,
}

#[derive(Clone, Debug)]
pub struct ElvesGroup {
    pub rucksacks: Vec<Rucksack>
}

impl ElvesGroup {
    fn new(rucksacks: Vec<Rucksack>) -> Self {
        ElvesGroup { rucksacks }
    }


    pub fn get_repeated_item(&self) -> String {
        let mut existing_chars: HashMap<String, usize> = std::collections::HashMap::new();

        for rucksack in self.rucksacks.clone() {
            let mut aux = rucksack.items.clone().chars().map(|x| String::from(x)).collect::<Vec<String>>();
            aux.sort();
            aux.dedup();

            for item in aux.clone() {
                if existing_chars.contains_key(&String::from(item.clone())) {
                    let existing_value = existing_chars.get(&String::from(item.clone())).unwrap();
                    existing_chars.insert(String::from(item), existing_value + 1);
                }
                else {
                    existing_chars.insert(String::from(item), 1);
                }
            }

        }

        let aux = existing_chars.iter().find(|(_character, ocurrences)| **ocurrences == self.rucksacks.len()).unwrap();

        return aux.0.clone();
    }

    pub fn get_item_type_priority(&self, item_type: &String) -> i32 {
        let chars = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ").chars().collect::<Vec<char>>();
        let item_priorities: Vec<i32> = (1..53).collect();

        return item_priorities[chars.iter().position(|&x| String::from(x) == *item_type).unwrap()];
    }

    pub fn get_badge_priority(&self) -> i32 {
        for rucksack in self.rucksacks.clone() {
            println!("{:?}", rucksack.items);
        }
        println!("{:?}", self.get_repeated_item());
        println!("{:?}", self.get_item_type_priority(&self.get_repeated_item()));
        self.get_item_type_priority(&self.get_repeated_item())
    }
}

impl Rucksack {
    fn new(items: String) -> Self {
        Rucksack { items }
    }
}

pub fn get_input() -> Vec<ElvesGroup> {
    let input = vec!["dtddvvhwttHJhwdhJPddhwJGppmGjgpQgTjQplQpTljwpg","BfzSzRSVVMVNRMDDNBSNSnfBmbrglGQbmNpQggFjpgpbQlQb","ZSBffLnVZdCCPJjhhL","RGCZpWWWFlHQQbgvFssg","jLnMzjnrnjjNjhrjdwbHscsVVgDVQPvPwh","nfJnLMLzjJMtnjNnnBbZtBWBqqbTTTBRpT","nddlhBtqTBqTVSlBtmCmVcRVmZggfWbcZc","jDjvPrPSNPwrDNRWbbgWCjRRCcWm","DzDwSpFrvrvFPQLzQnsqztBthTJnGJqlsJ","gssGmzwgRgsNmTsqgDnDJnbDHHhhzFdDDh","WQVFjMWrVQrVvVVjVctSSLSMZhnJZPBnbdnhbnHZZBDJBh","VCtcccVQLrfvrSlGmfTfNgfmlFgm","DsmfsBbNNZhDWsbmWmNbbPDHLFjcctjjGcnZGzncnctcGH","SwVQJrjVwpgSVRpjpVRrlTMCFFCLCFFcHzzGMcHrtHHH","ppVwTwSwpwvSlSlJTjVVbPhsvvBssWsNfsqWPvWs","BJwqwJtqqDDDrGDnPFzPFfpphD","TgZscCHQLSHgZcfMzpjFFjzsshfj","LcNlTVQCCVLLZTLNvpRtpvBBvRJmNB","bDBGQBBCTTNPGPPwPzcHfVHrDtLWLVrWVjjHWr","gpssqqsqlMFfLZQWftjVpr","lvqqFMRlFcQvbzCNCG","fhhMDdPhWMJMWvhhSfwRSGlzFbSFNlzw","LcqTCqcgZqjTggVjcwbFRwbDBTzbRGRwTS","cHLpZgnCHpQsDdsmQp","jwStJjJhtgJStpgwJMggQWqQTNTfNTWfbNNMCCNG","zRZnFPRZPVncPGVFRlRmGHCTqfCCPCHHfLfbTQCbTq","lnVmFZRZDnRVBFZcrZlhjpggvppthGhphpwprS","lcttSptHHllQbMcsrltSQGpvNBzpgWBBBDDGWzvgLz","PdjPVRFhFqFjRRCjzvRWnWLBLgbBBLzg","hhCCFbPTmjPdhZjhPhZCmTjjMsrJSfHrcmHJrHHmlcJSsmft","WhWnLZSSnSzQQhfLLNSfmDHrCFDDHtpjGGtTGQCG","gJbJBcMVwJlRRdbwvwJBVtjdtHHTmptpHTCtTFrFCp","JwwgvRMJlvJwgqgvqRMcnWWhLPzzsqfnZWnfWWnj","zdwTSvzHMvVSzDCtZhtGmbTGhm","lcBHfFjjgtsmDCgshD","cJPBnqNFnLfHJFPqljclqJzQvSSVWvSnMwvSzSWWdMWM","pNJMcZrsZDLDcbcccMpQffHqvgdwdFFmdmqwvqmgmzsw","hhnWjTTStRCGSMgvvgvdqvdFjvVz","hCTSWhPGttTCGBWMRlTCMSnPBDJpbDfDpNpbbNJfJDJbpJpN","lbcQcSNFchhQNqHLLqhLqrMpqM","WfsnsszPWfBBVpHdprrpdnGL","WTzWfwjtTBzwwBDzmfSSQmmbFZcpQNcbZZbv","PwSJSlmtPPgwgmHhPPvRvGHLRLQRBQGCQVGb","rnsFDnnfGGRWQRnW","dfTTfTFdfrfFFFzQFPJPSSlhqtllNPzgwS","MMbTFZrcrGZMDqNStWScDtzS","dvWmhQggQvCnfnqPqDnDjnfP","lgvdvLClWCQlgdhlrMBBHpGlwbHHGH","CQHgQpPdCQpsCpzRwSVRSzRZwZ","JbNBbcbrJvbJnqVznwwTzrzz","wNbfLvvfDNNBHPFLhddFsQss","VVzqvwzpqvzqNVVHGNqjHpNfSQDWdWwJdPWrWccdQrWrrDdd","nLcbtBRtBhcnWSJQlJSgll","tFbLLLRRhMtsBMtRCRsLCMBVjjvHTNjHHjzcvFFppGHzTT","QCPrPWNPlWjGGZqGmvdPGd","JgpHpSfphhfpVmBSgnTvdtddGvZVdvddDv","LhphBfHpSwSwfHcMgfpmBWWWbsNCjFWsljNbbjlLjb","QJmQbRmdfmdSQRQZSJltTltNvTrtDtrlftDD","wpZcHVwwMgBpWMVgWpHLphztDvvGvDPlnGvDLlNrDPnNPl","McgWFWHHHzVpMgZQFqbjsdjqqRCq","JPhLFfMJDLQnjNCvWWpdjjdM","crSwnwVnwSRBcNBNjjWCdC","GlbTGbsSzrtbmbfhnJQP","fDLSWVDRHHfVWHgPcZlDlZbbQhBcZQbb","jrmFmprTpFztmddjdjrpvBQlQZGhQbTsQbGcQbcbQs","nvqdpmjFnwpLSWlfnVNnWl","nZBRbBJzznNNCnJZwnBSCJMcpcTpcwhcqhmsmWMwFWLL","jQfvjgtfvPlHHqWpvWThpWqWch","VljjjgjQjrTDlDgrHtVCbnJZzNzNbnRNNJZrJR","MQtJnttlMLlJQsNhQrVVrFVWRRbbVFdJDD","vGjvzmjzgHqSjjSzmSGHTWbfDFWrbFzFfdDVrfRWDb","qPRqvTSPggqGgHCmllnCNLtnhcnnsnnw","zrlZsQMFrsgQFMMjMCbjVDCTCW","NqHNRdBppcJJcTpdmRfHThpdDWDtvbWVtbLjWbttWqqCCbLt","mhJpJHTJmBhcJhwhgwzsQwSSlzQQzGlZ","TvsszlvnzRRVTqzVrqrjjZGPfQPFqPqG","mcNhDNchppWmWSNhdSmSCQNjPFjrfGjrgPFCrgFPgPgrLf","SDddWpdMWSwNDmMNwlJRQwJlsVRRvzlsHt","DTtggjsFFFTlPJhvctBqBqSRmSMBSRnmnRcm","fGfwZdrbHVLdbGdHHwwQGVwBBCMMfvCNRNSMMMSRBmmRCN","dGZzGHGVVbvHvHwbzpGbHLrwFDDFTtsglhFspgJshslTDJjT","CbzspssWwCPcvvplrfqfDCJrDqdllB","LjttnjNTNGgQQJdBrffTwB","nVtLSgggjFwtMczhvzpZbSZW","HCzCHHvWthWFHhssWCVmnqZrnqVrmrmgnbrqmN","wPPGBjQQGwGbSlSLwgnpnrBZnBBmnMNnMN","jTTbJlJjPPLPGHHTthhhHcFWTT","qRdvvPDrCpzPHzcdrrcRqtbJJgjhgtWjJgbWJtgCFb","GTwGwNscLllGTZmGSTZGlSBMnhggjbgtgbtbsgWFFMhbMF","ZSQBSmlmzcrdQRqz","cSpTRphwwghRfgSScqPpnDqDCjDjJJJJDvDLCvvn","BVmmQFQBQVNBVmsWlbQFGBBlCHTJznzHLHvvCnjjNLHJDLHD","MFsZZMbBGblbQTmQsFsQMMfPcPcwSpwtStgPphZtctPc","QZbbZBdjPBjbQQbZnSSltlfwWvlvwNtNjwFMMN","DLVqTJqpSVtfsptwfWpv","rcRRVVTSbPQBPrBZ","tjSgSjLFSnVjDWRsQj","lcdqhfFpqZGpZqznrVRWPrnWRVBsVG","FHddNNNHwTHMHvvS","qCSDSQlwBHNbgJrHnLJH","GRpRpRfnmRWWVWgVrF","jhdZjpnvGfTZZQPlCtqQQSsS","FMZSGWWBrZjMBZMrBWMGjjZDnCRqpgPnbRwPbFnvvqFnDR","QHcpfVVslfdVlQclcctqRgqgbsCwbCwPCCCPwD","NLHfLhclmmhdfNNpfQMBmZWBrJMmZWBzMrjj","pBMpRgBMQwzRthmzLC","HPcJvrvDbjvrFDcvWrHfHfWHmdddtdTLztmtdtfllmNdNhNz","DvPFDvnPJLngQsggMGGQ","BbcFHvbhhDbbTSvZmwwgJPPlDlZldd","prCrNLMNgWWJBdrJ","fQMLCfLLtpqsNNMnnfBhcBSVGbhhhcqVbcjc","ZchcZZjmmNpgmJtgmM","RLrHllWrQZQGlBpbGFGFFM","RQnLHrqPLnZHzqjfVPcvVTfCvPTC","fMtwjfMwrbjfGrtrpPGrwpNNVNVqcbdVqHZTFNbcHSNL","mgzvDnJmnJhFJHSTNqZLHncHLS","vRzhzslJFhRffPPQMjGtGl","VMMNjWppQVwzNWrZdrrtMCMZCtMT","ngDScLcvPPgDPDGhGDPGSHVbHTHmZtTSrBHZbZBmBb","LhlglLghnVlplswJjs","bGJQZZTQQLJJbQZlTZLjCGQTsDhWFhmshhvjWVFVVrgtDsst","NScqwHcwwnnzBwqPqqsmVNhgsDDVtsghrFFg","pcrcwnpcffrcBzfbCRLpRLMMRlRLQl","hzCzCzpRgCzzzCctNsNWNqsZqZhPqNPb","TdBwmdrrrDmvwTvqNsSRssPlsWsq","FDBRRHDMTmBfmrmngnpjGgVptMgLCp","ZPLLnSPMFGvFZMSvHhDhqHfqvfqbDW","GgcppCgBcrQBBgplrVddhDqqqfdHgWdfqb","CcCjQszmGBQjrcCwCmCccPwPTPnMPTnMJSMMRZSPJL","LcVVcqqSHRLzRnCfNnGzNW","LZPPdljlCggMjgNM","PTvwlPtwtlJvZTQvbcHppFLHVVTcFssF","fpWzvzNgWJBVfBJzWzBVJNzWbZcbHhlbthjlrrPrjZZPHZhJ","hRDmGCFDwQnStncrjnccHcMP","GmmsGRmFTsFwSCsRQDsCSqqpfvfgzddWggvqdpfBWzVh","wjRBFljJGDFwwlGGpBSjGDtwTVtTgHHHsHHsVTVzsHqq","CPLNPdbWvbMWbcmvPNdLVqtsHqgCqHChZhhsVsHt","PWcPfPvmvNQbbWdWpJjJBDptGnDFjftn","mFFmJpDMmmnJFjWDVclsSpcflSsQwSsc","HrjNNjHNfVwLNSSl","tdZbhjHZHPbdCTvbbhhrGbbHMFmRMvnRRFmmvJMDmgJDJMnq","szJZhshbsfZJjbttchPctdTnWnRWVWMMnBdLRpMnBz","SrNwvDSwrCmnVRvjpWLBBn","ggGmgNFrgSDwmNgrCmtPsZPsjQGsqPcsqqJP","gjSWSjJSWrWzppzW","MCMzHNGNqHfscsFtrtwscVcr","qGHNGNHLCnLmTCHfMMmNTzzldzgJlJZZgJljgTdD","QGTQtQzTmdTsGTLcdFTGzdtBBjtwvBBJDvDMHJgjJvww","lPlqsZWnDJjZvZgV","ShCfCRnWGFsRRRrF","lwGtndCrrmGCwdmhzQrBzrHvLVggPgHv","fjMjDZJqSDJfJqDNDjJffjZLHPHHFvVFzHBLgLFpFpBSgL","MsTZWRNZfJZZqMGVGhhlhhccRnhC","MMvncqvcHcSnsdzzgvdfQjpljpQVTdDQDRTRlVpQ","wLCrNtBFFHHThRlH","bPJtHmCWssqgGPvq","LvTLsmDWvTWqTsmqjRTmjwgdwgnMHMMFgdtHmBmFVn","rlSCJzCSfpGGlhznQdnwFhtHgBFwtV","SGZJJSSrVfCbGJLjPsWbvjRsPTqR","pNqVVDCMVMBpqJVdMNHrccGHrtNtTFFFrQ","hwmllWbvvbnPvbSvtrFhhJzzHztcTztT","WSnbnPbbbvlWlRvnsqqMgLRMjLgVLCJdRV","GphVTGVMtQwtJmtCJP","FRRsBBsFqRNZNNrgqBdRfCZvbmPgmQzJQPnmJbJmQPJPPmwj","RNqsFrRfZZsZWvNqWRFvrBZvWhhCGVplhlWTlTpSCLpMhWMD","RZRjgbZHjjhsSnRsZstDRStsTVpFhBqFphMqPPpTFQVMPFTM","zrcGJwNNdwJrfNdJWvGdJzdTlTFlqTVPFTVFPPBpqNTbBP","WwLdLGfrRLStCZbD","mrmTqJWTvDDppTDb","DGzBfCzNDzdMwnLlbn","FVZPFZFFZPgjmWZsDtsq","TpnFTnFRCgRgldMRnDnRcrcdbdPBHbtPqbVcccrH","WNWLfQQmfhhSNwmrcbSVqPtbZDZcPb","LQhwLQvQvNfJhJRDMGFRlCMDMD","vLFTDmjVvLgnNHPphN","lMClGCmsRdCnPzCccngCpz","dlGZwRsRrRwswGsdSbbZSbVDrVBmDWWWFJrTrFvFTmqV","SGsZRqGLWLLtZRHRRcLHGTlJjzgJpjzTpNTNJNWpTm","MPMPvFFvFBrPPDPMQMPChjgpNpSNTmmmpNlTDljlTz","vnhrvMvnhSRqqLqnfn","mGFrlBmFQNQFljhqqqqbmHMsTPRbWWCsLMWRsb","wnwtvpwVzDVpvzzwZppnctMLtMPWWCstTsWTsTLffRRW","vwDJgZnvZJFqgLBFGqgl","QdGltnWNWqTdqQWvWsMJcrTcFcrgshJRMs","BzPLCDPzzzzCCLLfCBzfSDmLMrDJMglrcRbbhRsFhMrRJcsM","fjSzwwHfSzPzfCVBHlpdjGnZqnZptqQWjGvG","VbJZbgVzvzmhQpQWpQzhDp","tHPPcGcFBlCctCGtGcBBNlDLMGfMLwWfwwqMLLJwQWwp","dCHTPTPJdTBFPdrZjgsjrjnmdgms","JJpBvJQBZVvcFqqnsWdWvjsn","DCfbDbTtbgfCSHqqNdFMPhPDFnPPDWsPjM","bTmzTNCTNmfqTgJQcpLrpZLzVlVL","dtTLntTjzTftnmwnqGGQHNmm","SWbShCPMBgBRRFSFtRZZmm","DlJPCJCgPWhttzpvdjcpVl","WdzsNvWMzNsMHWddWCVffqmSmScLPvLPgLgLPplrrPmL","BtnzbnBhbwttwtZlmmlgcwSrLgmmpm","bFhQtbGBTnjBBbjTtFBbVDzddDDfjdDDqNWVjWHj","ppmtpgLLZLCbMQvQQThdtrvPhV","BBlHBwHRjHqBzzbHHqjjQdDQTDhPQDvnQlrQDQvr","HGjFzwHNczbzRFcGzHGFSJSpspsmpssMLLSZCppmfs","MpGrMMMcTsHMVHcvbwwmmcRSmDmDmv","zCNptqCBQQLCNLCzbfvSvbSzSDRDSmSv","CNNqNgNQJNgQtCqLlllZdZhTrThsnHpVVssPTsGP","jhSGcShDrLcLLFcw","MVzQvQNZVLHvHPdhLW","qzhhQlVbgqjmSjJDsgmR","CFzSPCgcsVVzFgzSCsBJwjdwJtNllnwglJlp","QrvbqWvvLbmvDMMmbdwFWpNNwwwwptjJWn","RZRZZqvvvDbDHCRTGchHFSGG","SszgPSPPVltDlqtz","WfTdTBdQdFnWBBBhBhNjVJtpNsVlDDDHHJWp","hQhrLFsBwdQPggbRgPwRMg","frRppMMDMpDnJfprnZhrrhpzWgvvGCvvFzWFvzvVVWFGJB","TcmLwTsccqwqbPwsdwqdTPSvBvzzztvggVvQCGWQCLBvCv","sswNjscwmqjwSssjdZNMfHHlHhfrnrgnfR","JpBJBdmdzZzzpngmbCnlqnNbNM","MMTHGccLTLvwRMlRnnQnbblnRnSs","vVGtvMcjLVGHfHDrPPWZppBpJpfZZZ","FGJtlttPdPtGFldlPRGpJTVzSBSSggHgJjVmBMHjJm","rhbvqrQLrWqrWLLfqbjjgNmVNSgzTmNgNS","hsffZQqnqCfZzlPPGlRlcwDs","HDDdZpcFwHFRFcZqDctpRDHpwTCVwjrBTQTBLBLBJJBjjQTJ","ldlMzhlPshPbLrrVrQQCMQjB","glzNfWlvbHqSdNNNcF","jZCMtnZZHCZwBWMwCwtMmfPFfvHDvzHFLPmFDfvh","RcrQdRRdGTzGvDGmfgjh","TsQscdQsQNTNqQQpRrRVCCBMMJJWMMVNVjnNJM","zVPWhVzLzWBWHZnlqBllqlpRbGNdffscGNdbDRnNSfcG","MtvSFQQwMcpsGRNGFR","vvTwJJSgmCSMmjVPPJWWhzllWLVV","RjdfnJfmbVvVJVFQcs","rZDZGBBZVvLZLHFW","qPzTDPlVrjNgfCdmPd","bcjmQPrnbmVmsLVrLrjmcHGRWlZHHRwHpZRHWWwH","nFhqzFqJzDJfvfSFqFfGHWZZHGRJRWHZWdpWwZ","hBCtDSSFCTqCCFzSnzMrLNmrMNPTNMQPMmNL","qvNBSJVDJGGVSJbVDDVhDbbqPjpWpWzWrnpWvvWPMjnWnpWz","mlTltwcwMWTPfNTN","CtCwFmCgmcmlRFmFCtRCHgmDJsbBhVqsbBHVDbNHDHJqqb","csBFBsLrBGBWcgLcBvRgpRhbwRwlbQwbwQgD","DCqmDmtTRtRlhdlh","qnCmTNPmmCnSSzmzNzGLzLccGDBzGrBLvvcW","FjfBjHnHzPFwhvFFqh","bjRpGsNsPqQvPclb","NWGGWGrrZVZjsCLmDMMgzgrSnzSm","MDgmmsNCmZMWmHCZLrvnLBBjPLVlPVbW","zcJGQwJdFRnrBVzqzvPr","hTQwhJwcfTFddFdGSfcRQQGFsggsgsHHnSmgsgsmgCnHNZpC","BPfwzfsgsvfszvBRbQpttRVpJbJpVg","LhTmHLbmbcFTFrWCbFqhFHLHVRpVtQpZVVDVprnDMJtJQnVZ","TGWWbTFFGTqlHhqhSdNdNfNSldjjBfjv","zCzpWTccHlWcPzMljMttbJfjmlfm","DqqQVZZqVsqJnbbnmjbJJQ","ZRmDZsSgVmGLsVqsLDFvrcccHrcTWCgWHBCHcCWp","cvGlQMtQlPtQWWMlcGsrFwFdbgdbdGGDCDCwdd","VChVZNBVjTTfhNTFgzrzrJgSdzgzwf","THThZTqZRHZRqNVZNTVLjRCMmQsntQctMnsPmMmMcWtLMQ","pNRHrbNlNnRLNpMMMTrcGcGTcccz","ZttBmsJmZdjsvTTvvdBMjDhfMGWGDfDfcScjfD","CmtTtwvtCsgllNHPPFbLpC","NpQcvwwRHvdfRvQsNfBQNvfRhVmVMqsZMmMshjMMtWZtMmrm","CGHbSSzFLSSHzTnbLnCWMrtWMtjnZMhZrqZtqW","FzCPPzLbPgFJbHSPldNRpgNfvvccgvwf","nSjpnnhNchMQZMSScnshshncJCGwHGClwmHPZlJPTVZCwHJf","LvtzBTgLWgLPlPwHPLPJ","dTBDqRqFzzhQFhshhNhM","HjjdPsjnllHsbnnDnbTBzLBFBZLLpRFRcCHRFz","wqqWwQhQQMCQffqqhtwMGhpZFRRZvzWzFvBvpvmcRvZm","fGfghtNhthqJrQqMqMMSgDdbPjbssDbdSnjCdd","cqPwJJnnffBFqSfJFnDDPVplLdglGgLVjzGLdVSzVt","WHRTWNHsQTNbzsbCbTsvWrWtjlgVdLgLdvdgvmLjpGlgtm","ZMQrTbNHZNsHHrQCZrNDFzhwnMJcfnDhJPPPFh","LRCFbjNjbCZDmtmqmRRmLtFJBgWBBpvJMwBJvGjBBvMBgw","TTrlfHzccVllZhdQgdGMJWvgWgBndwpG","fVSshSVlsfslhsSHHSZtZZNmNFmtmbFCDF","SPGCBPDMtbcbCtchSMccDTTrrrTFTrsrMTWHTHFVWF","JmnzqVmmwwfpJpmdHRTRsdsTrFdrQp","LqwLgzJgnjqLwgGcVbtjDGjcVbhv","PQcMvrvMsvmdSPPVccmSJcSpGBWWWbBHfWWnfttJWnWJpJ","wDzqhjzmqRzDRwqDzNDbWtjWBBBtGbtHpHnnBf","zglRhDqqDZgRNmZQVCdcCPQvvdZv","RpVjRgvFjGBNWtBWFDtt","dcqQwlqMMsCLLfbgQmtD","snlgzsggTzSTSJTr","dLHhDdtlMngFcFsFLFzzsj","vWRGGRVrrWvvGQQJBRsmQzmsqnffqcNfNcfz","vSRVJBVBwTvWTnHphTgDgtMpDl","bvvGnnJbfPmfdgJJSVtwwCpTScVfNpSC","sjsZWDqBqqMRZsDjbWMVwtwNNcNtScRHpRRttp","hzhDqqWDzZzDZzZLQPJPdPnPvlrbGdlnFQ","PwWHTwzFvNHsNzmmMwzNWGQrCqCFjpZbpnGqrqnpbr","gRVRgJRJlDLSJddDccQVrtZnCqjndnrZdnqnqpdq","chhgSSJfQhRRcSSSSBLVfzmzHTNzMNsTNWHMMvMP","lftqSpBSvhlDBDlhBSczQGmcFMcMVVFMmGFWsm","rHLHTNdggsLLnwLHbTTgdrTMPPmMGWZGQQMzQVQFZQGM","gbJnrHHjnbrgLrRrHpBJvSBDDsfJsDtstq","dBTtFLTtVmpdLhMprSRSWMRSMR","QvJvQbjbCgCQRBhzzRsNWNBC","bjgGqQGbQnjGQgnQgbGgjJnDLHLdfPVtdDmLZdBFVVZttdTf"];
    let mut elves_groups = vec![];

    for chunk in input.iter().map(|&x| String::from(x)).collect::<Vec<String>>().chunks(3) {
        let mut rucksacks= vec![];
        for item in chunk {
            rucksacks.push(Rucksack::new(item.to_string()));
        }

        elves_groups.push(ElvesGroup::new(rucksacks));
    }

    return elves_groups;
}

pub fn get_answer() {
    let elves_groups = get_input();
 
    println!("Total sum of item types for elves group: {:?}", elves_groups.iter().map(|group| group.get_badge_priority()).collect::<Vec<i32>>().iter().sum::<i32>());
}