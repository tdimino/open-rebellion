
void __thiscall FUN_0058ce70(void *this,int *param_1,undefined4 param_2)

{
  bool bVar1;
  uint uVar2;
  undefined4 uVar3;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0064ffe8;
  pvStack_c = ExceptionList;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  uVar2 = (**(code **)(*param_1 + 4))();
  if ((uVar2 < 0x28) || (0x2f < uVar2)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar1) {
    *(undefined4 *)((int)this + 0x20) = 1;
  }
  if (((uint)param_1[9] >> 6 & 3) == *(uint *)((int)this + 0xc)) {
    uVar3 = 0x10;
  }
  else {
    uVar3 = 0xf;
  }
  (**(code **)(*param_1 + 0xac))(uVar3,param_2);
  ExceptionList = (void *)0x28;
  return;
}

