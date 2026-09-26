
void __fastcall FUN_0046a880(undefined4 *param_1)

{
  undefined *puVar1;
  int *piVar2;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  puStack_8 = &LAB_00631ce6;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0065a218;
  local_4 = 1;
  if ((undefined4 *)param_1[0x47] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x47])(1);
  }
  if ((int *)param_1[0x49] != (int *)0x0) {
    for (piVar2 = (int *)(**(code **)(*(int *)param_1[0x49] + 8))(); piVar2 != (int *)0x0;
        piVar2 = (int *)(**(code **)(*piVar2 + 0xc))()) {
      puVar1 = (undefined *)piVar2[0x15];
      if (puVar1 != (undefined *)0x0) {
        FUN_00619730();
        FUN_00618b60(puVar1);
      }
    }
    if ((undefined4 *)param_1[0x49] != (undefined4 *)0x0) {
      (*(code *)**(undefined4 **)param_1[0x49])(1);
    }
  }
  if ((undefined4 *)param_1[0x4a] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x4a])(1);
  }
  if ((void *)param_1[0x4b] != (void *)0x0) {
    FUN_005f5b20((void *)param_1[0x4b]);
    if ((undefined4 *)param_1[0x4b] != (undefined4 *)0x0) {
      (*(code *)**(undefined4 **)param_1[0x4b])(1);
    }
  }
  if ((undefined4 *)param_1[0x4c] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x4c])(1);
  }
  if ((void *)param_1[0x4d] != (void *)0x0) {
    FUN_005f5b20((void *)param_1[0x4d]);
    if ((undefined4 *)param_1[0x4d] != (undefined4 *)0x0) {
      (*(code *)**(undefined4 **)param_1[0x4d])(1);
    }
  }
  if ((undefined4 *)param_1[0x4e] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x4e])(1);
  }
  if ((undefined4 *)param_1[0x48] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x48])(1);
  }
  local_4 = local_4 & 0xffffff00;
  FUN_00601990((int)(param_1 + 0x50));
  local_4 = 0xffffffff;
  FUN_00606580(param_1);
  ExceptionList = pvStack_c;
  return;
}

