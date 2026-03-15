
void __fastcall FUN_00435d40(int param_1)

{
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0062d4eb;
  pvStack_c = ExceptionList;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  *(undefined4 *)(param_1 + 0xc) = 0;
  if (*(undefined4 **)(param_1 + 0x10) != (undefined4 *)0x0) {
    (**(code **)**(undefined4 **)(param_1 + 0x10))(1);
  }
  *(undefined4 *)(param_1 + 0x10) = 0;
  if (*(undefined4 **)(param_1 + 0x14) != (undefined4 *)0x0) {
    (**(code **)**(undefined4 **)(param_1 + 0x14))(1);
  }
  *(undefined4 *)(param_1 + 0x14) = 0;
  local_4 = 0xffffffff;
  FUN_00536ef0((undefined4 *)(param_1 + 0x24));
  ExceptionList = pvStack_c;
  return;
}

