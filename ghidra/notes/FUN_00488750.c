
undefined4 __thiscall FUN_00488750(void *this,uint param_1)

{
  bool bVar1;
  int iVar2;
  uint uVar3;
  void *pvVar4;
  undefined4 uVar5;
  undefined4 local_2c;
  undefined4 auStack_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00634b67;
  local_c = ExceptionList;
  bVar1 = false;
  local_2c = 0;
  if (*(uint *)((int)this + 0x28) == param_1) {
    uVar3 = *(uint *)((int)this + 0x14);
    if ((uVar3 & 2) == 0) {
      if ((uVar3 & 1) == 0) {
        ExceptionList = &local_c;
        pvVar4 = (void *)FUN_00618b70(0x4c);
        if (pvVar4 == (void *)0x0) {
          FUN_00619730();
        }
        local_4 = 0;
        if (pvVar4 == (void *)0x0) {
          pvVar4 = (void *)0x0;
        }
        else {
          pvVar4 = FUN_00440980(pvVar4,param_1);
        }
        local_4 = 0xffffffff;
        *(void **)((int)this + 0x20) = pvVar4;
        if (pvVar4 != (void *)0x0) {
          uVar5 = (**(code **)(*(int *)this + 0x18))();
          FUN_005f5600(*(void **)((int)this + 0x20),uVar5);
          *(undefined4 *)(*(int *)((int)this + 0x20) + 0x20) = *(undefined4 *)((int)this + 0x24);
          FUN_005397d0(auStack_28);
          local_4 = 1;
          FUN_004fd080(auStack_28);
          FUN_00539880((void *)(*(int *)((int)this + 0x20) + 0x30),auStack_28);
          local_4 = 0xffffffff;
          *(undefined4 *)((int)this + 0x18) = *(undefined4 *)(*(int *)((int)this + 0x20) + 0x18);
          *(uint *)((int)this + 0x14) = *(uint *)((int)this + 0x14) | 1;
          FUN_00539830((int)auStack_28);
        }
      }
      else {
        ExceptionList = &local_c;
        iVar2 = FUN_0041d0c0(uVar3);
        do {
          if (iVar2 == 0) {
            uVar3 = 0;
          }
          else {
            uVar3 = *(uint *)(iVar2 + 0x18);
          }
          if ((*(uint *)((int)this + 0x30) < uVar3) && (uVar3 < *(uint *)((int)this + 0x18))) {
LAB_004887e5:
            bVar1 = true;
          }
          else if (uVar3 == 0) {
            FUN_0048aa90(this,*(undefined4 **)((int)this + 0x20));
            *(uint *)((int)this + 0x14) = *(uint *)((int)this + 0x14) | 2;
            *(undefined4 *)((int)this + 0x20) = 0;
            goto LAB_004887e5;
          }
          if (iVar2 != 0) {
            iVar2 = *(int *)(iVar2 + 0x10);
          }
        } while (!bVar1);
      }
    }
    else if (*(uint *)((int)this + 0x28) <= *(uint *)((int)this + 0x2c)) {
      *(undefined4 *)((int)this + 0x18) = 0;
      local_2c = 1;
    }
  }
  else {
    *(uint *)((int)this + 0x28) = param_1;
  }
  ExceptionList = local_c;
  return local_2c;
}

