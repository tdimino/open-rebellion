
int __thiscall FUN_005eedc0(void *this,int param_1,int param_2,int param_3,int param_4)

{
  int iVar1;
  
  if (param_4 == 0) {
    iVar1 = FUN_005f5060((int)this);
  }
  else {
    iVar1 = FUN_005f5080((int)this);
  }
  while ((iVar1 != 0 &&
         ((((param_1 != 0 && (param_1 != *(int *)(iVar1 + 0x18))) ||
           ((param_2 != 0 && (param_2 != *(int *)(iVar1 + 0x1c))))) ||
          ((param_3 != 0 && (param_3 != *(int *)(iVar1 + 0x20)))))))) {
    if (param_4 == 0) {
      iVar1 = *(int *)(iVar1 + 0x10);
    }
    else {
      iVar1 = FUN_005f5c60(iVar1);
    }
  }
  if (((param_1 == 0) && (param_2 == 0)) && (param_3 == 0)) {
    iVar1 = 0;
  }
  return iVar1;
}

