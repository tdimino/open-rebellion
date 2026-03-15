
void __thiscall FUN_004c5d50(void *this,undefined4 param_1)

{
  undefined4 *puVar1;
  int *piVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063a9bb;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = (undefined4 *)FUN_00618b70(0x9c);
  local_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    piVar2 = (int *)0x0;
  }
  else {
    piVar2 = FUN_004ebae0(puVar1);
  }
  local_4 = 0xffffffff;
  if (piVar2 != (int *)0x0) {
    (**(code **)(*piVar2 + 8))(param_1);
    piVar2[0xd] = *(int *)((int)this + 0x14);
    FUN_004f57b0(this,piVar2);
  }
  ExceptionList = local_c;
  return;
}

