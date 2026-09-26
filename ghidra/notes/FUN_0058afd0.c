
int __thiscall FUN_0058afd0(void *this,uint *param_1,int param_2)

{
  int iVar1;
  int iVar2;
  
  iVar2 = param_2;
  *(undefined4 *)(param_2 + 4) = 0;
  *(undefined4 *)(param_2 + 8) = 0;
  if ((*param_1 >> 0x18 < 0x90) || (param_2 = 1, 0x97 < *param_1 >> 0x18)) {
    param_2 = 0;
  }
  FUN_00619730();
  if (param_2 != 0) {
    iVar1 = (int)this + (*param_1 & 0xffffff) * 0x324 + -0x13a0c;
    *(int *)(iVar2 + 4) = iVar1;
    if (iVar1 != 0) {
      *(int *)(iVar2 + 8) = iVar1;
    }
  }
  return param_2;
}

