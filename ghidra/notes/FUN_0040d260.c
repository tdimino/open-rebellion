
uint __thiscall FUN_0040d260(void *this,int *param_1,void *param_2)

{
  int *this_00;
  uint uVar1;
  undefined4 *puVar2;
  int iVar3;
  int iVar4;
  undefined4 *puVar5;
  bool bVar6;
  undefined4 local_20 [6];
  int iStack_8;
  
  this_00 = param_1;
  bVar6 = false;
  if (*(int *)((int)this + 4) != 0) {
    bVar6 = *(int *)((int)this + 8) != 0;
  }
  uVar1 = 0;
  if (bVar6) {
    uVar1 = FUN_004f6df0(param_1,&param_1);
  }
  if (uVar1 != 0) {
    puVar2 = (undefined4 *)FUN_005968e0(local_20,(uint)param_1);
    puVar5 = local_20;
    for (iVar4 = 8; iVar4 != 0; iVar4 = iVar4 + -1) {
      *puVar5 = *puVar2;
      puVar2 = puVar2 + 1;
      puVar5 = puVar5 + 1;
    }
    if (uVar1 != 0) {
      iVar4 = (**(code **)(*this_00 + 0x254))();
      iVar3 = (**(code **)(*this_00 + 0x244))();
      iStack_8 = iStack_8 - (iVar3 - iVar4);
      if (0 < iStack_8) {
        uVar1 = FUN_00503400(this_00,iStack_8,param_2);
        return uVar1;
      }
      if (iStack_8 < 0) {
        uVar1 = FUN_00503370(this_00,-iStack_8,0xc,param_2);
      }
    }
  }
  return uVar1;
}

