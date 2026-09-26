
undefined4 __thiscall FUN_0041cb30(void *this,uint *param_1,int param_2,int param_3,int *param_4)

{
  short sVar1;
  bool bVar2;
  int iVar3;
  undefined3 extraout_var;
  int iVar4;
  int iVar5;
  undefined4 local_4;
  
  local_4 = 0;
  iVar5 = *(int *)((int)this + 0x1c) - param_2;
  iVar4 = *(int *)((int)this + 0x20) - param_3;
  if (-1 < iVar5) {
    iVar3 = FUN_005fc0e0((int *)param_1);
    if ((iVar5 < iVar3) && (-1 < iVar4)) {
      iVar3 = FUN_005fc0f0((int *)param_1);
      if ((iVar4 < iVar3) &&
         ((*(short *)((int)this + 0x34) != 4 && (*(int **)((int)this + 0x2c) != (int *)0x0)))) {
        bVar2 = FUN_005fc040(*(int **)((int)this + 0x2c));
        if (CONCAT31(extraout_var,bVar2) != 0) {
          local_4 = 1;
          *param_4 = iVar5;
          iVar3 = FUN_005fc0e0(*(int **)((int)this + 0x2c));
          param_4[1] = iVar4;
          param_4[2] = iVar3 + iVar5;
          iVar3 = FUN_005fc0f0(*(int **)((int)this + 0x2c));
          param_4[3] = iVar3 + iVar4;
          sVar1 = *(short *)((int)this + 0x34);
          if (sVar1 != 2) {
            if (sVar1 == 3) {
              FUN_005fe040(*(void **)((int)this + 0x2c),*(undefined4 *)((int)this + 0x30));
              FUN_005fe3d0(*(void **)((int)this + 0x2c),param_1,iVar5,iVar4,0,0,0,0);
              return 1;
            }
            if (sVar1 != 5) {
              FUN_005fd0f0(*(void **)((int)this + 0x2c),param_1,iVar5,iVar4);
              return 1;
            }
            FUN_005fe040(*(void **)((int)this + 0x2c),*(undefined4 *)((int)this + 0x30));
            FUN_005fe640(*(void **)((int)this + 0x2c),param_1,iVar5,iVar4,0,0,0,0);
            return 1;
          }
          FUN_005fe040(*(void **)((int)this + 0x2c),*(undefined4 *)((int)this + 0x30));
          FUN_005fe140(*(void **)((int)this + 0x2c),param_1,iVar5,iVar4,0,0,0,0);
        }
      }
    }
  }
  return local_4;
}

