
void __thiscall FUN_005a8d30(void *this,int param_1,int *param_2)

{
  int iVar1;
  void *pvVar2;
  uint uVar3;
  
  (**(code **)(*param_2 + 0x54))(*(undefined4 *)(param_1 + 0xec));
  (**(code **)(*(int *)this + 0x28))
            (0x37,param_2[6],*(undefined4 *)(param_1 + 0xec),
             param_2[0x14] == *(int *)((int)this + 0x9ec));
  iVar1 = (**(code **)(*param_2 + 4))();
  if (iVar1 == 3) {
    iVar1 = FUN_005ad900(param_2[6]);
    if ((iVar1 == 0) && (param_2[0x14] == *(int *)((int)this + 0x9ec))) {
      iVar1 = param_2[6];
      pvVar2 = (void *)FUN_0059ed70();
      FUN_005a0410(pvVar2,iVar1);
      return;
    }
  }
  else {
    iVar1 = FUN_005ad8d0(param_2[6]);
    if ((iVar1 == 0) && (param_2[0x14] == *(int *)((int)this + 0x9ec))) {
      uVar3 = param_2[6];
      pvVar2 = (void *)FUN_0059ed70();
      FUN_005a0320(pvVar2,uVar3);
    }
  }
  return;
}

