
void __thiscall FUN_005f5100(void *this,int param_1)

{
  short sVar1;
  int iVar2;
  int iVar3;
  
  if (param_1 != 0) {
    do {
      iVar2 = FUN_005ee440(param_1);
      sVar1 = (short)iVar2;
      while (sVar1 < -1) {
        iVar2 = *(int *)(param_1 + 4);
        iVar3 = FUN_005ee440(iVar2);
        if (0 < (short)iVar3) {
          FUN_005f5190(this,iVar2);
        }
        param_1 = FUN_005f5200(this,param_1);
        iVar2 = FUN_005ee440(param_1);
        sVar1 = (short)iVar2;
      }
      while (1 < sVar1) {
        iVar2 = *(int *)(param_1 + 8);
        iVar3 = FUN_005ee440(iVar2);
        if ((short)iVar3 < 0) {
          FUN_005f5200(this,iVar2);
        }
        param_1 = FUN_005f5190(this,param_1);
        iVar2 = FUN_005ee440(param_1);
        sVar1 = (short)iVar2;
      }
      param_1 = *(int *)(param_1 + 0xc);
    } while (param_1 != 0);
  }
  return;
}

