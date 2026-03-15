
uint __thiscall
FUN_00567880(void *this,int param_1,uint param_2,int param_3,int param_4,int param_5,int param_6,
            void *param_7,int *param_8)

{
  bool bVar1;
  int iVar2;
  void *this_00;
  uint *puVar3;
  int iVar4;
  undefined4 *puVar5;
  undefined3 extraout_var;
  uint uVar6;
  undefined1 local_14 [8];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064b21f;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_8 = 0;
  if ((param_1 < 1) || (2 < param_1)) {
    uVar6 = 0;
  }
  else {
    if ((param_2 < 0x28) || (0x2b < param_2)) {
      uVar6 = 0;
    }
    else {
      uVar6 = 1;
    }
    FUN_00619730();
  }
  if (uVar6 != 0) {
    uVar6 = (uint)(*(int *)((int)this + 8) != 0);
    if (uVar6 != 0) {
      iVar2 = thunk_FUN_005f5060(*(int *)((int)this + 8));
      while ((iVar2 != 0 && (uVar6 != 0))) {
        this_00 = (void *)thunk_FUN_005f5060(iVar2 + 0x30);
        while ((this_00 != (void *)0x0 && (uVar6 != 0))) {
          puVar3 = (uint *)FUN_00567a70(this_00,local_14);
          if ((param_2 < *puVar3) || (puVar3[1] <= param_2)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          if (((bVar1) && (iVar4 = FUN_0053b860((int)this_00), iVar4 != 0)) &&
             (((param_1 == 1 && (*(int *)((int)this_00 + 0x40) != 0)) ||
              ((param_1 == 2 && (*(int *)((int)this_00 + 0x44) != 0)))))) {
            if (*param_8 < *(int *)((int)this_00 + 0x50)) {
              *param_8 = *(int *)((int)this_00 + 0x50);
            }
            iVar4 = param_4;
            if ((param_3 <= *(int *)((int)this_00 + 0x50)) &&
               (iVar4 = param_5, *(int *)((int)this_00 + 0x50) != param_3)) {
              iVar4 = param_6;
            }
            if (iVar4 != 0) {
              puVar5 = (undefined4 *)FUN_00618b70(0x30);
              if (puVar5 == (undefined4 *)0x0) {
                FUN_00619730();
              }
              local_4 = 0;
              if (puVar5 == (undefined4 *)0x0) {
                puVar5 = (undefined4 *)0x0;
              }
              else {
                puVar5 = FUN_0058ecc0(puVar5);
              }
              uVar6 = (uint)(puVar5 != (undefined4 *)0x0);
              local_4 = 0xffffffff;
              if (uVar6 != 0) {
                uVar6 = FUN_0058eda0(puVar5,this_00);
                if (uVar6 != 0) {
                  bVar1 = FUN_004f4270(param_7,puVar5);
                  uVar6 = CONCAT31(extraout_var,bVar1);
                  if (uVar6 != 0) goto LAB_00567a36;
                }
                if (puVar5 != (undefined4 *)0x0) {
                  (**(code **)*puVar5)(1);
                }
              }
            }
          }
LAB_00567a36:
          this_00 = *(void **)((int)this_00 + 0x10);
        }
        iVar2 = *(int *)(iVar2 + 0x10);
      }
    }
  }
  ExceptionList = local_c;
  return uVar6;
}

