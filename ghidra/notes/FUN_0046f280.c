
void __fastcall FUN_0046f280(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  puStack_8 = &LAB_006323c6;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0065a300;
  local_4 = 1;
  if ((undefined4 *)param_1[0x48] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x48])(1);
  }
  if ((void *)param_1[0x49] != (void *)0x0) {
    FUN_005f5b20((void *)param_1[0x49]);
    if ((undefined4 *)param_1[0x49] != (undefined4 *)0x0) {
      (*(code *)**(undefined4 **)param_1[0x49])(1);
    }
  }
  if ((undefined4 *)param_1[0x57] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x57])(1);
  }
  if ((undefined4 *)param_1[0x58] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x58])(1);
  }
  if ((undefined4 *)param_1[0x5a] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x5a])(1);
  }
  if ((undefined4 *)param_1[0x5b] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x5b])(1);
  }
  if ((undefined4 *)param_1[0x59] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x59])(1);
  }
  local_4 = local_4 & 0xffffff00;
  FUN_00601990((int)(param_1 + 0x4a));
  local_4 = 0xffffffff;
  FUN_00606580(param_1);
  ExceptionList = local_c;
  return;
}

