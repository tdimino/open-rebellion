
undefined4 __thiscall
FUN_004f8880(void *this,char *param_1,char *param_2,int param_3,char *param_4,int param_5)

{
  int iVar1;
  char *pcVar2;
  undefined4 uVar3;
  
  uVar3 = 1;
  iVar1 = FUN_006158b0();
  if (iVar1 != 0) {
    pcVar2 = s_TRUE_006a8ef4;
    if (param_3 == 0) {
      pcVar2 = s_FALSE_006a8eec;
    }
    uVar3 = FUN_004f8c60(this,param_1,param_2,pcVar2,&DAT_006b120c,param_4,param_5);
  }
  return uVar3;
}

