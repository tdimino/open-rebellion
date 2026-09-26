
undefined4 __thiscall
FUN_004f88d0(void *this,char *param_1,char *param_2,int param_3,int param_4,char *param_5,
            int param_6)

{
  int iVar1;
  char *pcVar2;
  char *pcVar3;
  undefined4 uVar4;
  
  uVar4 = 1;
  iVar1 = FUN_006158b0();
  if (iVar1 != 0) {
    if (param_3 == 0) {
      pcVar3 = s_None_006a8efc;
    }
    else if (param_3 == 1) {
      pcVar3 = s_Alliance_006a8f04;
    }
    else if (param_3 == 2) {
      pcVar3 = s_Empire_006a8f10;
    }
    else if (param_3 == 3) {
      pcVar3 = s_Neutral_006a8f1c;
    }
    else {
      pcVar3 = s_UNKNOWN__006a8f28;
    }
    if (param_4 == 0) {
      pcVar2 = s_None_006a8efc;
    }
    else if (param_4 == 1) {
      pcVar2 = s_Alliance_006a8f04;
    }
    else if (param_4 == 2) {
      pcVar2 = s_Empire_006a8f10;
    }
    else if (param_4 == 3) {
      pcVar2 = s_Neutral_006a8f1c;
    }
    else {
      pcVar2 = s_UNKNOWN__006a8f28;
    }
    uVar4 = FUN_004f8c60(this,param_1,param_2,pcVar3,pcVar2,param_5,param_6);
  }
  return uVar4;
}

