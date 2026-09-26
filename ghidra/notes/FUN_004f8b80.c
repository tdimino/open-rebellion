
undefined4 __thiscall
FUN_004f8b80(void *this,char *param_1,char *param_2,uint *param_3,uint *param_4,char *param_5,
            int param_6)

{
  int iVar1;
  char *pcVar2;
  char *pcVar3;
  undefined4 uVar4;
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063fea0;
  local_c = ExceptionList;
  uVar4 = 1;
  ExceptionList = &local_c;
  iVar1 = FUN_006158b0();
  if (iVar1 != 0) {
    FUN_005f2f50(local_18);
    local_4 = 0;
    FUN_005f2f50(local_24);
    local_4._0_1_ = 1;
    FUN_0051ccc0(param_3,local_18);
    FUN_0051ccc0(param_4,local_24);
    pcVar2 = (char *)FUN_00583c40((int)local_24);
    pcVar3 = (char *)FUN_00583c40((int)local_18);
    uVar4 = FUN_004f8c60(this,param_1,param_2,pcVar3,pcVar2,param_5,param_6);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_005f2ff0(local_24);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_18);
  }
  ExceptionList = local_c;
  return uVar4;
}

