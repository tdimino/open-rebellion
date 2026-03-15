
int __thiscall
FUN_005f6920(void *this,char *param_1,int param_2,void *param_3,int param_4,int param_5,
            void *param_6,undefined4 param_7)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  int *this_00;
  int iVar3;
  void *pvVar4;
  int iVar5;
  int local_28;
  void *local_24;
  int local_20;
  tagRECT local_1c;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00655806;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_24 = this;
  FUN_005f6b60((int)this);
  local_20 = 0;
  iVar3 = param_2;
  if ((param_2 != 0) || (iVar3 = DAT_006be328, DAT_006be328 != 0)) {
    local_28 = 0;
    param_2 = 0;
    *(int *)((int)this + 0xc) = param_5;
    bVar1 = FUN_005f6be0(param_1,&local_28,&param_2,param_5);
    iVar5 = param_4;
    pvVar4 = param_3;
    local_20 = CONCAT31(extraout_var,bVar1);
    iVar2 = FUN_005f6b40((int)param_3,param_4);
    if ((iVar2 != 0) && (*(HWND *)(iVar3 + 0x18) != (HWND)0x0)) {
      GetClientRect(*(HWND *)(iVar3 + 0x18),&local_1c);
      pvVar4 = (void *)(local_1c.right / 2 - ((uint)(local_28 * (int)param_6) >> 1));
      iVar5 = local_1c.bottom / 2 - ((uint)(param_2 * (int)param_6) >> 1);
    }
    param_5 = *(int *)(iVar3 + 0x1c);
    this_00 = (int *)0x0;
    if (local_20 == 0) {
      FUN_0060a9e0(param_1,s_PathName_unknown_to_VideoManager_006ac244);
      PostMessageA(*(HWND *)(iVar3 + 0x18),0x403,0,0);
    }
    else if (*(int *)((int)local_24 + 0xc) == 2) {
      param_6 = (void *)FUN_00618b70(0x1a4);
      uStack_4 = 0;
      if (param_6 == (void *)0x0) {
        this_00 = (int *)0x0;
      }
      else {
        this_00 = (int *)FUN_005f74c0(param_6,param_1,param_5,pvVar4,iVar5,local_28,param_2,iVar3,1)
        ;
      }
      uStack_4 = 0xffffffff;
      *(int **)((int)local_24 + 4) = this_00;
    }
    else if (*(int *)((int)local_24 + 0xc) == 1) {
      param_3 = (void *)FUN_00618b70(0x1c4);
      uStack_4 = 1;
      if (param_3 == (void *)0x0) {
        pvVar4 = (void *)0x0;
      }
      else {
        pvVar4 = FUN_005f6cd0(param_3,param_1,param_5,pvVar4,iVar5,local_28,param_2,iVar3,1,param_7)
        ;
      }
      uStack_4 = 0xffffffff;
      *(void **)((int)local_24 + 8) = pvVar4;
      if ((pvVar4 != (void *)0x0) && ((void *)0x1 < param_6)) {
        *(void **)((int)pvVar4 + 0x19c) = param_6;
      }
      this_00 = *(int **)((int)local_24 + 8);
    }
    if (this_00 != (int *)0x0) {
      FUN_005ffce0(this_00,0);
      (**(code **)(*this_00 + 4))(5);
      local_20 = 1;
      *(undefined4 *)((int)local_24 + 0x10) = 1;
    }
  }
  ExceptionList = local_c;
  return local_20;
}

