
void __thiscall FUN_00603aa0(void *this,int param_1,uint param_2)

{
  undefined4 uVar1;
  bool bVar2;
  void *pvVar3;
  undefined3 extraout_var;
  int iVar4;
  int *piVar5;
  uint uVar6;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00656346;
  local_c = ExceptionList;
  uVar1 = *(undefined4 *)((int)this + 0x1c);
  uVar6 = 0;
  ExceptionList = &local_c;
  if (param_2 != 0) {
    do {
      pvVar3 = (void *)FUN_00618b70(0x28);
      piVar5 = (int *)0x0;
      local_4 = 0;
      if (pvVar3 != (void *)0x0) {
        piVar5 = FUN_005fbd20(pvVar3,uVar1,uVar6 + param_1,2);
      }
      local_4 = 0xffffffff;
      if ((piVar5 != (int *)0x0) &&
         (bVar2 = FUN_005fc040(piVar5), CONCAT31(extraout_var,bVar2) != 0)) {
        pvVar3 = (void *)FUN_00618b70(0x10);
        local_4 = 1;
        if (pvVar3 == (void *)0x0) {
          iVar4 = 0;
        }
        else {
          iVar4 = FUN_0060f3c0(pvVar3,piVar5);
        }
        local_4 = 0xffffffff;
        FUN_0060f320((void *)((int)this + 0x114),iVar4);
      }
      uVar6 = uVar6 + 1;
    } while ((uVar6 & 0xffff) < param_2);
  }
  *(undefined4 *)((int)this + 0x120) = *(undefined4 *)((int)this + 0x11c);
  ExceptionList = local_c;
  return;
}

