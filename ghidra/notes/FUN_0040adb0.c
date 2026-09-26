
uint FUN_0040adb0(void *param_1,undefined4 param_2,int param_3)

{
  void *this;
  LPBYTE pBVar1;
  char *pcVar2;
  uint uVar3;
  undefined4 local_30 [3];
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00629edf;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this = (void *)FUN_00618b70(0x28);
  if (this == (void *)0x0) {
    FUN_00619730();
  }
  local_4 = 0;
  if (this == (void *)0x0) {
    DAT_006b1218 = (void *)0x0;
  }
  else {
    DAT_006b1218 = FUN_0041dd20(this,param_2,param_3);
  }
  local_4 = 0xffffffff;
  uVar3 = 0;
  if (((DAT_006b1218 != (void *)0x0) && (uVar3 = 0, DAT_006b1218 != (void *)0x0)) &&
     (uVar3 = *(uint *)((int)DAT_006b1218 + 4), uVar3 != 0)) {
    FUN_005f2f50(local_30);
    local_4 = 1;
    FUN_005f3c20(local_30);
    pBVar1 = FUN_00401760();
    if (pBVar1 != (LPBYTE)0x0) {
      FUN_005f35b0(local_18,&DAT_006a7c4c);
      local_4._0_1_ = 2;
      FUN_005f35b0(local_24,(char *)pBVar1);
      local_4._0_1_ = 3;
      FUN_005f30d0(local_30,(int)local_24);
      FUN_005f30d0(local_30,(int)local_18);
      local_4._0_1_ = 2;
      FUN_005f2ff0(local_24);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_005f2ff0(local_18);
      FUN_00618b60(pBVar1);
    }
    pcVar2 = FUN_005f49e0(0x610);
    FUN_005f35b0(local_18,pcVar2);
    local_4._0_1_ = 4;
    FUN_005f30d0(local_30,(int)local_18);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_005f2ff0(local_18);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_30);
    if (uVar3 != 0) {
      FUN_0041df40(DAT_006b1218,param_1);
    }
  }
  ExceptionList = local_c;
  return uVar3;
}

