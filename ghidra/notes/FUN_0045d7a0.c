
void __fastcall FUN_0045d7a0(undefined4 *param_1)

{
  undefined *puVar1;
  undefined4 *puVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00630cc8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_00659fa8;
  local_4 = 0;
  if ((undefined4 *)param_1[0x49] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x49])(1);
  }
  if ((undefined4 *)param_1[0x4a] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x4a])(1);
  }
  if ((undefined4 *)param_1[0x4b] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x4b])(1);
  }
  puVar1 = (undefined *)param_1[0x4c];
  if (puVar1 != (undefined *)0x0) {
    FUN_00601990((int)puVar1);
    FUN_00618b60(puVar1);
  }
  puVar1 = (undefined *)param_1[0x48];
  if (puVar1 != (undefined *)0x0) {
    FUN_00601990((int)puVar1);
    FUN_00618b60(puVar1);
  }
  puVar2 = (undefined4 *)param_1[0x53];
  if (puVar2 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar2);
    FUN_00618b60((undefined *)puVar2);
  }
  if ((undefined4 *)param_1[0x54] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x54])(1);
  }
  if ((undefined4 *)param_1[0x47] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x47])(1);
  }
  if ((undefined4 *)param_1[0x4f] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x4f])(1);
  }
  if ((undefined4 *)param_1[0x50] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x50])(1);
  }
  if ((undefined4 *)param_1[0x51] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x51])(1);
  }
  if ((void *)param_1[0x4d] != (void *)0x0) {
    FUN_005f5b20((void *)param_1[0x4d]);
    if ((undefined4 *)param_1[0x4d] != (undefined4 *)0x0) {
      (*(code *)**(undefined4 **)param_1[0x4d])(1);
    }
  }
  FUN_00603830(4);
  FUN_00603830(5);
  local_4 = 0xffffffff;
  FUN_00606580(param_1);
  ExceptionList = local_c;
  return;
}

