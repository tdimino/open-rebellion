
int * __thiscall FUN_004bced0(void *this,int *param_1)

{
  int iVar1;
  int *piVar2;
  
  piVar2 = (int *)0x0;
  *param_1 = 0;
  switch(*(undefined4 *)((int)this + 0x1c)) {
  case 1:
    if ((*(uint *)((int)this + 0x20) & 0x60000000) == 0) {
      if ((*(uint *)((int)this + 0x20) & 0x10000000) == 0) {
        *(undefined4 *)((int)this + 0x1c) = 2;
      }
      else {
        FUN_0047aff0(this);
        *param_1 = 1;
      }
    }
    else {
      piVar2 = FUN_0047b250(this);
      *(undefined4 *)((int)this + 0x1c) = 0;
      *param_1 = 1;
    }
    break;
  case 2:
    FUN_0047b110(this);
    *(undefined4 *)((int)this + 0x1c) = 3;
    break;
  case 3:
    (**(code **)(*(int *)this + 0x48))();
    *(undefined4 *)((int)this + 0x1c) = 4;
    break;
  case 4:
    (**(code **)(*(int *)this + 0x4c))();
    *(undefined4 *)((int)this + 0x1c) = 5;
    break;
  case 5:
    (**(code **)(*(int *)this + 0x50))();
    *(undefined4 *)((int)this + 0x1c) = 6;
    break;
  case 6:
    iVar1 = FUN_004bd0a0((int)this);
    if (iVar1 != 0) {
      *(undefined4 *)((int)this + 0x1c) = 7;
      break;
    }
    goto LAB_004bcfb5;
  case 7:
    piVar2 = FUN_004bd570((int)this);
    if (((*(uint *)((int)this + 0x20) & 8) != 0) && ((*(uint *)((int)this + 0x20) & 0x10) != 0)) {
      *(undefined4 *)((int)this + 0x1c) = 8;
      break;
    }
    goto LAB_004bcfb5;
  case 8:
    piVar2 = FUN_0047b360((int)this);
LAB_004bcfb5:
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

