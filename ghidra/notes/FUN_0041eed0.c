
void __fastcall FUN_0041eed0(undefined4 *param_1)

{
  undefined4 *puVar1;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0062b8c9;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_00658ad0;
  param_1[0x25] = &PTR_LAB_00658aa0;
  puVar1 = (undefined4 *)param_1[0x2b];
  local_4 = 5;
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  if ((HDC)param_1[0x47] != (HDC)0x0) {
    if ((HGDIOBJ)param_1[0x49] != (HGDIOBJ)0x0) {
      SelectObject((HDC)param_1[0x47],(HGDIOBJ)param_1[0x49]);
    }
    if ((HGDIOBJ)param_1[0x48] != (HGDIOBJ)0x0) {
      SelectObject((HDC)param_1[0x47],(HGDIOBJ)param_1[0x48]);
    }
    DeleteDC((HDC)param_1[0x47]);
  }
  local_4._0_1_ = 4;
  FUN_005f5870(param_1 + 0x58);
  local_4._0_1_ = 3;
  FUN_005f3e00(param_1 + 0x43);
  local_4._0_1_ = 2;
  FUN_005f3e00(param_1 + 0x40);
  local_4._0_1_ = 1;
  FUN_005f2ff0(param_1 + 0x28);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_0060a460((undefined4 *)(-(uint)(param_1 != (undefined4 *)0x0) & (uint)(param_1 + 0x25)));
  local_4 = 0xffffffff;
  FUN_005ffb60(param_1);
  ExceptionList = pvStack_c;
  return;
}

