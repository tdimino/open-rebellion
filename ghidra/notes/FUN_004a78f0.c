
void __fastcall FUN_004a78f0(undefined4 *param_1)

{
  undefined4 *puVar1;
  undefined *puVar2;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  puStack_8 = &LAB_00637f76;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0065be30;
  local_4 = 1;
  if ((undefined4 *)param_1[0x58] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x58])(1);
  }
  puVar1 = (undefined4 *)param_1[0x5f];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  puVar1 = (undefined4 *)param_1[0x60];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  puVar2 = (undefined *)param_1[0x5b];
  if (puVar2 != (undefined *)0x0) {
    FUN_00601990((int)puVar2);
    FUN_00618b60(puVar2);
  }
  puVar2 = (undefined *)param_1[0x5c];
  if (puVar2 != (undefined *)0x0) {
    FUN_00601990((int)puVar2);
    FUN_00618b60(puVar2);
  }
  puVar1 = (undefined4 *)param_1[0x5d];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  puVar1 = (undefined4 *)param_1[0x5e];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  if ((void *)param_1[0x5a] != (void *)0x0) {
    FUN_005f5b20((void *)param_1[0x5a]);
    if ((undefined4 *)param_1[0x5a] != (undefined4 *)0x0) {
      (*(code *)**(undefined4 **)param_1[0x5a])(1);
    }
  }
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004ac2f0(param_1);
  ExceptionList = local_c;
  return;
}

