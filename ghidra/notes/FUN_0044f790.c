
void __fastcall FUN_0044f790(undefined4 *param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0062fa18;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_00659d28;
  local_4 = 0;
  if ((undefined4 *)param_1[0x4b] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x4b])(1);
  }
  if ((void *)param_1[0x4c] != (void *)0x0) {
    FUN_005f5b20((void *)param_1[0x4c]);
    if ((undefined4 *)param_1[0x4c] != (undefined4 *)0x0) {
      (*(code *)**(undefined4 **)param_1[0x4c])(1);
    }
  }
  if ((undefined4 *)param_1[0x4d] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x4d])(1);
  }
  if ((undefined4 *)param_1[0x48] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x48])(1);
  }
  if ((undefined4 *)param_1[0x49] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x49])(1);
  }
  if ((undefined4 *)param_1[0x4a] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x4a])(1);
  }
  if ((undefined4 *)param_1[0x47] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x47])(1);
  }
  local_4 = 0xffffffff;
  FUN_00606580(param_1);
  ExceptionList = local_c;
  return;
}

