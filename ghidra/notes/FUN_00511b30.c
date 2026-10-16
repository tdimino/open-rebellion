
undefined4 __thiscall FUN_00511b30(void *this,int param_1,void *param_2)

{
  undefined4 uVar1;
  int iVar2;
  int iVar3;
  
  uVar1 = 1;
  if (param_1 != 0) {
    iVar2 = FUN_0053f9c0(0x311,this,1,param_2);
    iVar3 = FUN_0050ac10(this,0,param_2);
    if ((iVar3 != 0) && (iVar2 != 0)) {
      return 1;
    }
    uVar1 = 0;
  }
  return uVar1;
}

