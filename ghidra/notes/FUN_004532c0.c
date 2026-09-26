
void __fastcall FUN_004532c0(undefined4 *param_1)

{
  undefined *puVar1;
  undefined4 *puVar2;
  int iVar3;
  undefined4 *puVar4;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0062fe3e;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_00659e68;
  local_4 = 2;
  if ((undefined4 *)param_1[0x54] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x54])(1);
  }
  if ((undefined4 *)param_1[0x55] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x55])(1);
  }
  if ((undefined4 *)param_1[0x56] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x56])(1);
  }
  puVar4 = (undefined4 *)param_1[0x62];
  if (puVar4 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar4);
    FUN_00618b60((undefined *)puVar4);
  }
  puVar4 = (undefined4 *)param_1[0x65];
  if (puVar4 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar4);
    FUN_00618b60((undefined *)puVar4);
  }
  puVar4 = (undefined4 *)param_1[0x66];
  if (puVar4 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar4);
    FUN_00618b60((undefined *)puVar4);
  }
  puVar1 = (undefined *)param_1[0x67];
  if (puVar1 != (undefined *)0x0) {
    FUN_00601990((int)puVar1);
    FUN_00618b60(puVar1);
  }
  puVar4 = (undefined4 *)param_1[0x99];
  if (puVar4 != (undefined4 *)0x0) {
    FUN_0060db40(puVar4);
    FUN_00618b60((undefined *)puVar4);
  }
  puVar4 = (undefined4 *)param_1[0x60];
  if (puVar4 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar4);
    FUN_00618b60((undefined *)puVar4);
  }
  puVar4 = (undefined4 *)param_1[0x61];
  if (puVar4 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar4);
    FUN_00618b60((undefined *)puVar4);
  }
  puVar1 = (undefined *)param_1[0x5f];
  if (puVar1 != (undefined *)0x0) {
    FUN_00601990((int)puVar1);
    FUN_00618b60(puVar1);
  }
  puVar4 = (undefined4 *)param_1[0x57];
  if (puVar4 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar4);
    FUN_00618b60((undefined *)puVar4);
  }
  if ((undefined4 *)param_1[0x5b] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x5b])(1);
  }
  puVar4 = (undefined4 *)param_1[99];
  if (puVar4 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar4);
    FUN_00618b60((undefined *)puVar4);
  }
  puVar4 = (undefined4 *)param_1[100];
  if (puVar4 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar4);
    FUN_00618b60((undefined *)puVar4);
  }
  if ((void *)param_1[0x5e] != (void *)0x0) {
    FUN_005f5b20((void *)param_1[0x5e]);
    if ((undefined4 *)param_1[0x5e] != (undefined4 *)0x0) {
      (*(code *)**(undefined4 **)param_1[0x5e])(1);
    }
  }
  puVar4 = param_1 + 0x95;
  iVar3 = 3;
  do {
    puVar2 = (undefined4 *)*puVar4;
    if (puVar2 != (undefined4 *)0x0) {
      FUN_005fbfa0(puVar2);
      FUN_00618b60((undefined *)puVar2);
    }
    *puVar4 = 0;
    puVar4 = puVar4 + 1;
    iVar3 = iVar3 + -1;
  } while (iVar3 != 0);
  FUN_00458500((int)param_1);
  local_4._0_1_ = 1;
  FUN_00618d20(param_1 + 0x68,0x38,3,FUN_00453260);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004ac2f0(param_1);
  ExceptionList = local_c;
  return;
}

