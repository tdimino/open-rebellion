
void FUN_00409bd0(void)

{
  LPBYTE pBVar1;
  char *pcVar2;
  undefined4 local_30 [3];
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00629c88;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f2f50(local_30);
  local_4 = 0;
  FUN_005f3c20(local_30);
  pBVar1 = FUN_00401760();
  if (pBVar1 != (LPBYTE)0x0) {
    FUN_005f35b0(local_18,&DAT_006a7c4c);
    local_4._0_1_ = 1;
    FUN_005f35b0(local_24,(char *)pBVar1);
    local_4._0_1_ = 2;
    FUN_005f30d0(local_30,(int)local_24);
    FUN_005f30d0(local_30,(int)local_18);
    local_4._0_1_ = 1;
    FUN_005f2ff0(local_24);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_005f2ff0(local_18);
    FUN_00618b60(pBVar1);
  }
  pcVar2 = FUN_005f49e0(0x610);
  FUN_005f35b0(local_18,pcVar2);
  local_4._0_1_ = 3;
  FUN_005f30d0(local_30,(int)local_18);
  local_4._0_1_ = 0;
  FUN_005f2ff0(local_18);
  pcVar2 = FUN_005f49e0(0x613);
  FUN_005f35b0(local_18,pcVar2);
  local_4._0_1_ = 4;
  FUN_005f30d0(local_30,(int)local_18);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_005f2ff0(local_18);
  pcVar2 = (char *)FUN_00583c40((int)local_30);
  FUN_005f4bd0(pcVar2);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_30);
  ExceptionList = local_c;
  return;
}

