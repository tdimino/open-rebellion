
undefined4 FUN_005531b0(int *param_1,void *param_2)

{
  bool bVar1;
  uint uVar2;
  uint local_14;
  uint local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00648fa0;
  pvStack_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  FUN_00520580(param_2,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  local_14 = 0x90;
  local_10 = 0x98;
  local_4 = 1;
  uVar2 = (**(code **)(*param_1 + 4))();
  if ((uVar2 < local_14) || (local_10 <= uVar2)) {
    local_4._0_1_ = 2;
    uVar2 = (**(code **)(*param_1 + 4))();
    if ((uVar2 < 0x98) || (0x9f < uVar2)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    local_4._0_1_ = 1;
    FUN_00619730();
    if (!bVar1) {
      local_4._0_1_ = 3;
      uVar2 = (**(code **)(*param_1 + 4))();
      if ((uVar2 < 8) || (0xf < uVar2)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      local_4._0_1_ = 1;
      FUN_00619730();
      if (!bVar1) {
        local_4._0_1_ = 4;
        uVar2 = (**(code **)(*param_1 + 4))();
        if ((uVar2 < 0x14) || (0x1b < uVar2)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        local_4 = CONCAT31(local_4._1_3_,1);
        FUN_00619730();
        if (!bVar1) goto LAB_005532ea;
      }
    }
  }
  bVar1 = true;
LAB_005532ea:
  local_4 = 0xffffffff;
  FUN_00619730();
  if (!bVar1) {
    local_14 = 1;
    local_10 = 0x25;
    local_4 = 5;
    FUN_00520580(param_2,&local_14);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = pvStack_c;
  return 1;
}

