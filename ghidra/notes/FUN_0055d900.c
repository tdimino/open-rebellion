
bool __cdecl FUN_0055d900(int param_1,int param_2,undefined4 *param_3)

{
  undefined4 *puVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  bool bVar4;
  bool bVar5;
  undefined4 uStack_54;
  undefined4 local_38;
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c;
  undefined4 local_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00649dc6;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = FUN_00562ae0(local_28);
  local_4 = 0;
  uStack_54 = 0x55d93b;
  FUN_00562b80(param_3,puVar1);
  local_4 = 0xffffffff;
  FUN_00562b30();
  if ((param_1 < 1) || (bVar5 = true, 3 < param_1)) {
    bVar5 = false;
  }
  bVar4 = false;
  if ((bVar5) && (bVar4 = param_1 != 3, bVar4)) {
    *param_3 = 0;
    bVar5 = param_2 == 0;
    if (bVar5) {
      local_38 = 0x92;
      local_34 = 0x94;
      puVar1 = &local_38;
      local_4 = 2;
    }
    else {
      local_30 = 0x90;
      local_2c = 0x92;
      puVar1 = &local_30;
      local_4 = 1;
    }
    FUN_00520580(&uStack_54,puVar1);
    FUN_0055de00((int)param_3);
    local_4 = 1;
    if (bVar5) {
      FUN_00619730();
    }
    local_4 = 0xffffffff;
    if (!bVar5) {
      FUN_00619730();
    }
    param_3[3] = param_1;
    param_3[5] = param_1;
    if (param_2 == 0) {
      uVar2 = DAT_006bb614;
      uVar3 = DAT_006bb6d0;
      if (param_1 != 1) {
        uVar2 = DAT_006bb5f4;
        uVar3 = DAT_006bb6c4;
      }
    }
    else {
      uVar2 = DAT_006bb6a8;
      uVar3 = DAT_006bb624;
      if (param_1 == 1) {
        uVar2 = DAT_006bb66c;
        uVar3 = DAT_006bb6c0;
      }
    }
    param_3[4] = uVar3;
    param_3[6] = uVar2;
  }
  ExceptionList = local_c;
  return bVar4;
}

