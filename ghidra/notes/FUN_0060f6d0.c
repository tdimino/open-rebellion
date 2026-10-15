
void __thiscall FUN_0060f6d0(void *this,undefined2 param_1)

{
  int *piVar1;
  int iVar2;
  
  piVar1 = *(int **)((int)this + 0x98);
  if (piVar1 == (int *)0x0) {
    return;
  }
  switch(param_1) {
  case 0:
  case 100:
    iVar2 = (**(code **)(*piVar1 + 0xc))();
    break;
  case 1:
  case 0x65:
    iVar2 = (**(code **)(*piVar1 + 0x10))();
    break;
  case 2:
    (**(code **)(*(int *)this + 0x44))();
    return;
  case 3:
    (**(code **)(*(int *)this + 0x48))();
  default:
    goto switchD_0060f6f3_caseD_4;
  }
  if (iVar2 != 0) {
    FUN_0060b3d0(this,iVar2);
    return;
  }
switchD_0060f6f3_caseD_4:
  return;
}

