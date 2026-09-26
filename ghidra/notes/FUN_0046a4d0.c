
void __thiscall FUN_0046a4d0(void *this,int param_1,undefined4 param_2)

{
  int iVar1;
  BOOL BVar2;
  uint uVar3;
  void *pvVar4;
  
  switch(param_1) {
  case 0xd:
    if (*(int *)((int)this + 0x118) != 2) {
      return;
    }
    if (*(int *)((int)this + 0x170) == 0) {
      return;
    }
    pvVar4 = (void *)0x0;
    iVar1 = (**(code **)(**(int **)(*(int *)((int)this + 0x170) + 0x68) + 0x10))();
    if (2 < iVar1) {
      if (iVar1 < 5) {
        uVar3 = 0x97;
      }
      else {
        if (iVar1 != 5) goto LAB_0046a601;
        uVar3 = 0x98;
      }
      pvVar4 = (void *)FUN_00604500((void *)((int)this + 0x6c),uVar3);
    }
LAB_0046a601:
    if (pvVar4 != (void *)0x0) {
      FUN_00603710(pvVar4);
      FUN_00603780(pvVar4);
      return;
    }
    break;
  default:
    (**(code **)(**(int **)((int)this + 0x20) + 0x14))
              (*(int **)((int)this + 0x20),0x100,param_1,param_2);
    break;
  case 0x1b:
    if (*(int *)((int)this + 0x118) == 1) {
      (**(code **)(*(int *)this + 0x30))();
      return;
    }
    (**(code **)(*(int *)this + 0x48))(0x65,0,0);
    return;
  case 0x25:
  case 0x27:
    iVar1 = *(int *)(*(int *)((int)this + 0x160) + 0x94);
    if (iVar1 == 0) {
      uVar3 = 0;
    }
    else {
      uVar3 = *(uint *)(iVar1 + 0x24);
    }
    iVar1 = FUN_00604500((void *)(*(int *)((int)this + 0x160) + 0x6c),uVar3);
    if (param_1 == 0x25) {
      iVar1 = FUN_005f5c60(iVar1);
    }
    else {
      iVar1 = *(int *)(iVar1 + 0x10);
    }
    if (iVar1 != 0) {
      do {
        BVar2 = IsWindowVisible(*(HWND *)(iVar1 + 0x18));
        if (BVar2 != 0) {
          FUN_0060d7e0(*(void **)((int)this + 0x160),*(uint *)(iVar1 + 0x24),1);
          return;
        }
        if (param_1 == 0x25) {
          iVar1 = FUN_005f5c60(iVar1);
        }
        else {
          iVar1 = *(int *)(iVar1 + 0x10);
        }
      } while (iVar1 != 0);
      return;
    }
    iVar1 = FUN_005f5060(*(int *)((int)this + 0x160) + 0x6c);
    if (iVar1 != 0) {
      FUN_0060d7e0(*(void **)((int)this + 0x160),*(uint *)(iVar1 + 0x24),1);
      return;
    }
    break;
  case 0x2e:
    (**(code **)(*(int *)this + 0x48))(0x91,0,0);
    return;
  }
  return;
}

