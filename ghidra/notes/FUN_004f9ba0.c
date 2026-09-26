
uint __thiscall FUN_004f9ba0(void *this,void *param_1)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  int iVar3;
  undefined *puVar4;
  undefined *this_00;
  uint uVar5;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063ff58;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  bVar1 = FUN_0053a010((int)this);
  uVar5 = CONCAT31(extraout_var,bVar1);
  if (uVar5 != 0) {
    iVar2 = FUN_00539fd0(this,0);
    iVar3 = FUN_005404d0(this,*(uint *)((int)param_1 + 8));
    if (iVar3 != 0) {
      puVar4 = FUN_004f62d0(iVar2);
      this_00 = FUN_004f62d0((int)this);
      iVar3 = FUN_005f3390(this_00,(int)puVar4);
      if (iVar3 != 0) {
        puVar4 = FUN_004f62d0((int)this);
        FUN_005f2f90(local_18,(int)puVar4);
        local_4 = 0;
        uVar5 = FUN_004fc680((int)this);
        if (uVar5 != 0) {
          puVar4 = FUN_004f62d0(iVar2);
          FUN_005f3090(*(void **)((int)this + 0x34),(int)puVar4);
          iVar2 = FUN_004fd3d0();
          if (iVar2 != 0) {
            iVar2 = FUN_004fd600(param_1,*(uint *)((int)this + 0x24) >> 4 & 3);
            if (iVar2 != 0) {
              iVar2 = *(int *)this;
              puVar4 = FUN_004f62d0((int)this);
              uVar5 = (**(code **)(iVar2 + 0x154))(local_18,puVar4,param_1);
            }
          }
        }
        local_4 = 0xffffffff;
        FUN_005f2ff0(local_18);
      }
    }
  }
  ExceptionList = local_c;
  return uVar5;
}

