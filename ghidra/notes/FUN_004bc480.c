
int * __thiscall FUN_004bc480(void *this,int *param_1)

{
  int iVar1;
  int *piVar2;
  
  piVar2 = (int *)0x0;
  *param_1 = 0;
  switch(*(undefined4 *)((int)this + 0x1c)) {
  case 1:
    if ((*(uint *)((int)this + 0x20) & 0x80000000) == 0) {
      *(undefined4 *)((int)this + 0x1c) = 3;
    }
    else {
      piVar2 = FUN_004bc520(this);
      *param_1 = 1;
    }
    break;
  case 2:
    iVar1 = (**(code **)(*(int *)this + 0x28))();
    if (iVar1 != 0) {
      *(undefined4 *)((int)this + 0x1c) = 4;
      break;
    }
    goto LAB_004bc4ed;
  case 3:
    FUN_004bc5d0((int)this);
    *(undefined4 *)((int)this + 0x1c) = 2;
    break;
  case 4:
    piVar2 = FUN_004bca20(this);
LAB_004bc4ed:
    *param_1 = 1;
    break;
  default:
    *(undefined4 *)((int)this + 0x1c) = 1;
  }
  if (*param_1 != 0) {
    *(undefined4 *)((int)this + 0x1c) = 0;
  }
  return piVar2;
}

