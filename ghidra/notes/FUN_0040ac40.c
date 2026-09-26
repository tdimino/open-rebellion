
uint FUN_0040ac40(void)

{
  undefined4 *puVar1;
  LPBYTE pBVar2;
  char *pcVar3;
  uint uVar4;
  undefined4 local_30 [3];
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00629e9f;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = (undefined4 *)FUN_00618b70(0x28);
  if (puVar1 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    DAT_006b1218 = (undefined4 *)0x0;
  }
  else {
    DAT_006b1218 = FUN_0041dce0(puVar1);
  }
  local_4 = 0xffffffff;
  uVar4 = 0;
  if (((DAT_006b1218 != (undefined4 *)0x0) && (uVar4 = 0, DAT_006b1218 != (undefined4 *)0x0)) &&
     (uVar4 = DAT_006b1218[1], uVar4 != 0)) {
    FUN_005f2f50(local_30);
    local_4 = 1;
    FUN_005f3c20(local_30);
    pBVar2 = FUN_00401760();
    if (pBVar2 != (LPBYTE)0x0) {
      FUN_005f35b0(local_18,&DAT_006a7c4c);
      local_4._0_1_ = 2;
      FUN_005f35b0(local_24,(char *)pBVar2);
      local_4._0_1_ = 3;
      FUN_005f30d0(local_30,(int)local_24);
      FUN_005f30d0(local_30,(int)local_18);
      local_4._0_1_ = 2;
      FUN_005f2ff0(local_24);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_005f2ff0(local_18);
      FUN_00618b60(pBVar2);
    }
    pcVar3 = FUN_005f49e0(0x610);
    FUN_005f35b0(local_18,pcVar3);
    local_4._0_1_ = 4;
    FUN_005f30d0(local_30,(int)local_18);
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_005f2ff0(local_18);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_30);
  }
  ExceptionList = local_c;
  return uVar4;
}

