
bool FUN_00555d30(int *param_1,uint *param_2,int *param_3,undefined4 *param_4)

{
  int iVar1;
  uint *puVar2;
  bool bVar3;
  uint uVar4;
  
  *param_4 = 0;
  bVar3 = true;
  iVar1 = (**(code **)(*param_1 + 0x38))();
  if (iVar1 != 0) {
    puVar2 = (uint *)((uint)param_1[9] >> 4 & 3);
    uVar4 = 1;
    iVar1 = (**(code **)(*param_1 + 0x34))();
    iVar1 = FUN_00555b30(iVar1,uVar4,puVar2,param_2,param_3);
    bVar3 = iVar1 != 0;
  }
  return bVar3;
}

