
void __thiscall FUN_005a90c0(void *this,int *param_1)

{
  int iVar1;
  
  if (param_1[0x5c] != 0xb) {
    FUN_005c5f20((void *)((int)this + 0x9d0),(int)param_1);
    FUN_005aa760(this,(int *)param_1[0x3b]);
    param_1[0x5c] = 9;
    iVar1 = (**(code **)(*param_1 + 0x20))();
    if (iVar1 == 0) {
      FUN_005b8d10((int)(param_1 + 0x106));
    }
    iVar1 = *(int *)((int)this + param_1[0x5a] * 4 + 8);
    if (*(int *)(iVar1 + 0x10) != 0) {
      *(undefined4 *)(iVar1 + 0x84) = 5;
    }
  }
  return;
}

