
void FUN_00485640(int param_1,undefined4 *param_2,int param_3,undefined4 *param_4)

{
  undefined4 *puVar1;
  int *piVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063479f;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = (undefined4 *)FUN_00618b70(0x30);
  if (puVar1 == (undefined4 *)0x0) {
    FUN_00619730();
  }
  local_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    piVar2 = (int *)0x0;
  }
  else {
    piVar2 = FUN_00440630(puVar1);
  }
  local_4 = 0xffffffff;
  if (piVar2 != (int *)0x0) {
    piVar2[9] = param_3;
    piVar2[8] = param_1;
    FUN_004fcd90(piVar2 + 10,param_4);
    FUN_004f26d0(piVar2 + 0xb,param_2);
    FUN_004855b0(piVar2);
    (**(code **)*piVar2)(1);
  }
  ExceptionList = local_c;
  return;
}

