
int __thiscall FUN_0053b3e0(void *this,undefined4 param_1,undefined4 param_2,int param_3)

{
  bool bVar1;
  undefined3 extraout_var;
  char *pcVar2;
  char *pcVar3;
  
  bVar1 = FUN_0053a010((int)this);
  if ((CONCAT31(extraout_var,bVar1) != 0) && (DAT_006b9048 != 0)) {
    switch(param_2) {
    case 0:
      pcVar3 = s_NONE_006aac64;
      break;
    case 1:
      pcVar3 = s_WAITING_006aac58;
      break;
    case 2:
      pcVar3 = s_PROCESSING_006aac4c;
      break;
    case 3:
      pcVar3 = s_DONE_006aac40;
      break;
    case 4:
      pcVar3 = s_IDLE_006aac34;
      break;
    default:
      pcVar3 = s_UNKNOWN_006aac28;
    }
    switch(param_1) {
    case 0:
      pcVar2 = s_NONE_006aac64;
      break;
    case 1:
      pcVar2 = s_WAITING_006aac58;
      break;
    case 2:
      pcVar2 = s_PROCESSING_006aac4c;
      break;
    case 3:
      pcVar2 = s_DONE_006aac40;
      break;
    case 4:
      pcVar2 = s_IDLE_006aac34;
      break;
    default:
      pcVar2 = s_UNKNOWN_006aac28;
    }
    FUN_004f8c60(this,s__ProcFacilProcFacilStateNotif__006aabf8,s_ProcFacilState_006aac18,pcVar3,
                 pcVar2,(char *)0x0,param_3);
  }
  return CONCAT31(extraout_var,bVar1);
}

