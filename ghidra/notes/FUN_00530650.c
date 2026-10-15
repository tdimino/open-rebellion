
int FUN_00530650(void *param_1,void *param_2)

{
  int iVar1;
  
  iVar1 = 1;
  if ((*(byte *)((int)param_1 + 0x60) & 2) == 0) {
    iVar1 = FUN_0053a860(param_1,1,param_2);
  }
  return iVar1;
}

