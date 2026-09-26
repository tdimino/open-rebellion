
int * __thiscall FUN_004adb70(void *this,undefined4 *param_1)

{
  undefined4 *puVar1;
  int iVar2;
  int *piVar3;
  
  puVar1 = param_1;
  piVar3 = (int *)0x0;
  *param_1 = 0;
  switch(*(undefined4 *)((int)this + 8)) {
  case 1:
    FUN_004af6d0((int)this);
    *(undefined4 *)((int)this + 8) = 2;
    return (int *)0x0;
  case 2:
    iVar2 = (**(code **)(*(int *)this + 0x34))();
    *(uint *)((int)this + 8) = (-(uint)(iVar2 != 0) & 0xfffffffe) + 5;
    return (int *)0x0;
  case 3:
    piVar3 = (int *)(**(code **)(*(int *)this + 0x38))();
    *(undefined4 *)((int)this + 8) = 4;
    return piVar3;
  case 4:
    piVar3 = FUN_004b0120((int)this);
    *(undefined4 *)((int)this + 8) = 5;
    return piVar3;
  case 5:
    iVar2 = FUN_004b04c0((int)this);
    *(uint *)((int)this + 8) = (-(uint)(iVar2 != 0) & 0xfffffff8) + 0xe;
    return (int *)0x0;
  case 6:
    piVar3 = FUN_004b06b0(this,&param_1);
    *(uint *)((int)this + 8) = (-(uint)(param_1 != (undefined4 *)0x0) & 7) + 7;
    return piVar3;
  case 7:
    iVar2 = (**(code **)(*(int *)this + 0x3c))();
    *(uint *)((int)this + 8) = (-(uint)(iVar2 != 0) & 5) + 8;
    return (int *)0x0;
  case 8:
    iVar2 = (**(code **)(*(int *)this + 0x40))();
    *(uint *)((int)this + 8) = (-(uint)(iVar2 != 0) & 3) + 10;
    return (int *)0x0;
  case 9:
    iVar2 = (**(code **)(*(int *)this + 0x44))();
    *(uint *)((int)this + 8) = (iVar2 != 0) + 0xc;
    return (int *)0x0;
  case 10:
    iVar2 = (**(code **)(*(int *)this + 0x48))();
    *(uint *)((int)this + 8) = (-(uint)(iVar2 != 0) & 4) + 9;
    return (int *)0x0;
  default:
    *(undefined4 *)((int)this + 8) = 1;
    break;
  case 0xc:
    piVar3 = FUN_004b0c90(this);
    *(undefined4 *)((int)this + 8) = 0xe;
    return piVar3;
  case 0xd:
    piVar3 = FUN_004b0f90((int)this);
    *(undefined4 *)((int)this + 8) = 0xe;
    return piVar3;
  case 0xe:
    piVar3 = FUN_004b1070(this,&param_1);
    if (param_1 != (undefined4 *)0x0) {
      *(undefined4 *)((int)this + 8) = 0;
      *puVar1 = 1;
      return piVar3;
    }
  }
  return piVar3;
}

