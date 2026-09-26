
void __fastcall FUN_004a2820(undefined4 *param_1)

{
  undefined4 *puVar1;
  undefined *puVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_00637972;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0065bda0;
  local_4 = 3;
  if ((undefined4 *)param_1[0x56] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x56])(1);
  }
  if ((undefined4 *)param_1[0x57] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x57])(1);
  }
  if ((undefined4 *)param_1[0x58] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x58])(1);
  }
  if ((undefined4 *)param_1[0x59] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x59])(1);
  }
  puVar1 = (undefined4 *)param_1[0x5a];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  if ((undefined4 *)param_1[0x62] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x62])(1);
  }
  if ((undefined4 *)param_1[99] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[99])(1);
  }
  puVar1 = (undefined4 *)param_1[0x5e];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  puVar1 = (undefined4 *)param_1[0x5f];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  puVar2 = (undefined *)param_1[100];
  if (puVar2 != (undefined *)0x0) {
    FUN_00601990((int)puVar2);
    FUN_00618b60(puVar2);
  }
  puVar1 = (undefined4 *)param_1[0x60];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  puVar1 = (undefined4 *)param_1[0x61];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  if ((HDC)param_1[0x6b] != (HDC)0x0) {
    DeleteDC((HDC)param_1[0x6b]);
  }
  if ((HGDIOBJ)param_1[0x68] != (HGDIOBJ)0x0) {
    DeleteObject((HGDIOBJ)param_1[0x68]);
  }
  local_4._0_1_ = 2;
  FUN_006044d0(param_1 + 0x54);
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004ac2f0(param_1);
  ExceptionList = pvStack_c;
  return;
}

