
void __thiscall FUN_005c2990(void *this,int param_1,undefined4 param_2)

{
  void **ppvVar1;
  void *pvVar2;
  int iVar3;
  void *unaff_ESI;
  undefined4 uVar4;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00653ae1;
  local_c = ExceptionList;
  iVar3 = param_1;
  ExceptionList = &local_c;
  ppvVar1 = &local_c;
  if (4 < param_1 - 7U) {
LAB_005c2be8:
    ExceptionList = ppvVar1;
    if (*(int **)((int)this + 0x638) != (int *)0x0) {
      (**(code **)(**(int **)((int)this + 0x638) + 0x10))(iVar3);
    }
    ExceptionList = local_c;
    return;
  }
  do {
                    /* WARNING: Could not find normalized switch variable to match jumptable */
    switch(param_1) {
    case 7:
      if (*(undefined4 **)((int)this + 0x644) != (undefined4 *)0x0) {
        (**(code **)**(undefined4 **)((int)this + 0x644))(1);
        *(undefined4 *)((int)this + 0x644) = 0;
      }
      if (*(void **)((int)this + 0x640) != (void *)0x0) {
        FUN_005d95d0(*(void **)((int)this + 0x63c),*(void **)((int)this + 0x640));
        *(undefined4 *)((int)this + 0x638) = *(undefined4 *)((int)this + 0x63c);
        ExceptionList = local_c;
        return;
      }
      iVar3 = FUN_00595c40();
      if (iVar3 == 1) {
        uVar4 = 0x96;
      }
      else {
        uVar4 = 0xffffffe2;
      }
      FUN_005d9620(*(void **)((int)this + 0x63c),0x1e,uVar4,0x3e4ccccd);
LAB_005c2b44:
      *(undefined4 *)((int)this + 0x638) = *(undefined4 *)((int)this + 0x63c);
      ExceptionList = local_c;
      return;
    case 8:
      if (*(void **)((int)this + 0x640) != (void *)0x0) {
        FUN_005d95d0(*(void **)((int)this + 0x640),*(void **)((int)this + 0x63c));
        ExceptionList = local_c;
        return;
      }
      pvVar2 = (void *)FUN_00618b70(0x50);
      local_4 = 2;
      if (pvVar2 == (void *)0x0) {
        *(undefined4 *)((int)this + 0x640) = 0;
        ExceptionList = local_c;
        return;
      }
      pvVar2 = FUN_005d9580(pvVar2,*(int *)((int)this + 0x63c));
      *(void **)((int)this + 0x640) = pvVar2;
      ExceptionList = local_c;
      return;
    case 9:
      ppvVar1 = ExceptionList;
      if (*(int *)((int)this + 0x644) == 0) goto LAB_005c2be8;
      FUN_005c2990(this,7,param_2);
      iVar3 = 9;
      break;
    case 10:
      if (*(undefined4 **)((int)this + 0x644) != (undefined4 *)0x0) {
        (**(code **)**(undefined4 **)((int)this + 0x644))(1);
        *(undefined4 *)((int)this + 0x644) = 0;
        *(undefined4 *)((int)this + 0x638) = *(undefined4 *)((int)this + 0x63c);
        ExceptionList = unaff_ESI;
        return;
      }
      goto LAB_005c2b44;
    default:
      if (*(undefined4 **)((int)this + 0x644) != (undefined4 *)0x0) {
        (**(code **)**(undefined4 **)((int)this + 0x644))(1);
      }
      pvVar2 = (void *)FUN_00618b70(0x3c);
      local_4 = 0;
      if (pvVar2 == (void *)0x0) {
        pvVar2 = (void *)0x0;
      }
      else {
        pvVar2 = FUN_005d9a70(pvVar2,0x3f000000,param_2,*(int *)((int)this + 0x63c));
      }
      *(void **)((int)this + 0x644) = pvVar2;
      local_4 = 0xffffffff;
      if (*(int *)((int)this + 0x640) == 0) {
        pvVar2 = (void *)FUN_00618b70(0x50);
        local_4 = 1;
        if (pvVar2 == (void *)0x0) {
          pvVar2 = (void *)0x0;
        }
        else {
          pvVar2 = FUN_005d9580(pvVar2,*(int *)((int)this + 0x63c));
        }
        *(void **)((int)this + 0x640) = pvVar2;
      }
      *(undefined4 *)((int)this + 0x638) = *(undefined4 *)((int)this + 0x644);
      ExceptionList = local_c;
      return;
    }
  } while( true );
}

