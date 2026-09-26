
int * __thiscall FUN_00474390(void *this,undefined4 *param_1)

{
  undefined4 *puVar1;
  int iVar2;
  int *piVar3;
  
  puVar1 = param_1;
  piVar3 = (int *)0x0;
  *param_1 = 0;
  if (*(int *)((int)this + 0x38) != 0) {
    FUN_004747b0(this);
    *(undefined4 *)((int)this + 0x38) = 0;
  }
  switch(*(undefined4 *)((int)this + 0x1c)) {
  case 1:
    iVar2 = FUN_004744b0((int)this);
    *(uint *)((int)this + 0x1c) = (-(uint)(iVar2 != 0) & 0xfffffffa) + 8;
    break;
  case 2:
    piVar3 = FUN_004749b0((int)this);
    *(undefined4 *)((int)this + 0x1c) = 3;
    break;
  case 3:
    FUN_00474b90((int)this);
    *(undefined4 *)((int)this + 0x1c) = 4;
    break;
  case 4:
    piVar3 = FUN_00474c00(this,&param_1);
    if (param_1 != (undefined4 *)0x0) {
      *(undefined4 *)((int)this + 0x1c) = 5;
    }
    break;
  case 5:
    piVar3 = FUN_00474ce0((int)this);
    *(undefined4 *)((int)this + 0x1c) = 6;
    break;
  case 6:
    piVar3 = FUN_00474ff0((int)this);
    *(undefined4 *)((int)this + 0x1c) = 7;
    break;
  case 7:
    piVar3 = FUN_00475330((int)this);
    *(undefined4 *)((int)this + 0x1c) = 0;
    *puVar1 = 1;
    break;
  case 8:
    FUN_004742b0(this);
    *(undefined4 *)((int)this + 0x1c) = 0;
    *puVar1 = 1;
    break;
  default:
    *(undefined4 *)((int)this + 0x1c) = 1;
    *(undefined4 *)((int)this + 0x38) = 1;
  }
  if (piVar3 != (int *)0x0) {
    *(undefined4 *)((int)this + 0x38) = 1;
  }
  return piVar3;
}

