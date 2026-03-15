
void __thiscall FUN_004b1800(void *this,int *param_1)

{
  short sVar1;
  void *pvVar2;
  
  *param_1 = 0;
  pvVar2 = FUN_004f5940((void *)(*(int *)((int)this + 0xc) + 0x44),
                        (uint *)(*(int *)((int)this + 0x10) + 0x34));
  if ((pvVar2 != (void *)0x0) && (*param_1 == 0)) {
    if (*(int *)(*(int *)((int)this + 0x10) + 200) < 1) {
      sVar1 = FUN_005f50e0(*(int *)((int)this + 0x10) + 0x2c);
      if (sVar1 != 0) {
        *param_1 = 3;
      }
    }
    if ((((*param_1 == 0) && (*(int *)(*(int *)((int)this + 0x10) + 0xd0) < 1)) &&
        (0 < *(int *)(*(int *)((int)this + 0x10) + 0x98))) &&
       ((*(uint *)((int)pvVar2 + 0x2c) & 0x40008) != 0)) {
      *param_1 = 5;
    }
  }
  return;
}

