
void __fastcall FUN_005ffb60(undefined4 *param_1)

{
  undefined4 *puVar1;
  HWND pHVar2;
  HWND pHVar3;
  undefined4 *puVar4;
  undefined4 *puVar5;
  undefined4 *puVar6;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0065601c;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0066df18;
  local_4 = 3;
  FUN_006014d0((uint)param_1);
  if ((undefined *)param_1[0x12] != (undefined *)0x0) {
    FUN_00618b60((undefined *)param_1[0x12]);
  }
  if ((undefined *)param_1[0x13] != (undefined *)0x0) {
    FUN_00618b60((undefined *)param_1[0x13]);
  }
  if (param_1[0x21] != 0) {
    FUN_00600d30(param_1,(int)param_1);
  }
  puVar4 = DAT_006be598;
  puVar5 = (undefined4 *)0x0;
  if (DAT_006be598 != (undefined4 *)0x0) {
    do {
      puVar1 = (undefined4 *)puVar4[4];
      puVar6 = puVar4;
      if (puVar4[6] == param_1[6]) {
        puVar6 = puVar1;
        if (puVar5 != (undefined4 *)0x0) {
          puVar5[4] = puVar1;
          puVar6 = DAT_006be598;
        }
        DAT_006be598 = puVar6;
        puVar6 = puVar5;
        if (puVar4 != (undefined4 *)0x0) {
          (**(code **)*puVar4)(1);
        }
      }
      puVar4 = puVar1;
      puVar5 = puVar6;
    } while (puVar1 != (undefined4 *)0x0);
    if ((DAT_006be598 != (undefined4 *)0x0) &&
       (pHVar2 = (HWND)DAT_006be598[6], pHVar3 = GetCapture(), pHVar2 != pHVar3)) {
      SetCapture((HWND)DAT_006be598[6]);
    }
  }
  FUN_006011f0((int)param_1);
  local_4._0_1_ = 2;
  FUN_0060f060(param_1 + 0x22);
  local_4._0_1_ = 1;
  FUN_0060f060(param_1 + 0x1d);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_006044d0(param_1 + 0x1b);
  local_4 = 0xffffffff;
  FUN_005f5c50(param_1);
  ExceptionList = pvStack_c;
  return;
}

