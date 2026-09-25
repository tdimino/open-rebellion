
undefined4 __thiscall FUN_00468fb0(void *this,int param_1)

{
  int iVar1;
  int iVar2;
  int *piVar3;
  uint uVar4;
  void *pvVar5;
  uint uVar6;
  uint uVar7;
  undefined4 uVar8;
  undefined2 local_50;
  undefined2 local_4e;
  undefined2 uStack_4c;
  undefined2 uStack_4a;
  undefined2 uStack_48;
  undefined2 uStack_46;
  undefined2 uStack_44;
  undefined2 uStack_42;
  undefined2 uStack_40;
  undefined2 uStack_3e;
  undefined2 uStack_3c;
  undefined2 uStack_3a;
  undefined2 uStack_38;
  undefined2 uStack_36;
  undefined2 uStack_34;
  undefined2 uStack_32;
  undefined2 uStack_30;
  undefined2 uStack_2e;
  undefined4 local_2c [2];
  int local_24;
  tagRECT tStack_1c;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00631c20;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_006037f0(7);
  uVar8 = 0;
  if (param_1 == 0x65) {
    iVar2 = 2 - (uint)(*(int *)((int)this + 0x118) != 1);
  }
  else {
    iVar2 = param_1;
    if (param_1 == 0x68) {
      iVar2 = 3;
    }
  }
  if (iVar2 == 0) {
    uVar8 = 0;
  }
  else {
    iVar1 = *(int *)((int)this + 0x118);
    if (iVar2 == iVar1) {
      uVar8 = 0;
    }
    else {
      *(int *)((int)this + 0x118) = iVar2;
      if (iVar2 == 1) {
        *(uint *)(*(int *)((int)this + 0x168) + 0xf4) =
             *(uint *)(*(int *)((int)this + 0x168) + 0xf4) & 0xfffeffff;
        FUN_006075e0(this,0x32);
        (**(code **)(**(int **)((int)this + 0x168) + 4))(5);
        (**(code **)(**(int **)((int)this + 0x160) + 4))(5);
        pvVar5 = (void *)((int)this + 0x6c);
        piVar3 = (int *)FUN_00604500(pvVar5,0x91);
        (**(code **)(*piVar3 + 4))(5);
        piVar3 = (int *)FUN_00604500(pvVar5,0x90);
        (**(code **)(*piVar3 + 4))(5);
        piVar3 = (int *)FUN_00604500(pvVar5,0x96);
        (**(code **)(*piVar3 + 8))();
        piVar3 = (int *)FUN_00604500(pvVar5,0x9a);
        (**(code **)(*piVar3 + 8))();
        piVar3 = (int *)FUN_00604500(pvVar5,0x98);
        (**(code **)(*piVar3 + 8))();
        piVar3 = (int *)FUN_00604500(pvVar5,0x99);
        (**(code **)(*piVar3 + 8))();
        (**(code **)(**(int **)((int)this + 0x1dc) + 8))();
        pvVar5 = (void *)FUN_00604500((void *)(*(int *)((int)this + 0x160) + 0x6c),0x79);
        uStack_4c = 0x8010;
        uStack_4a = DAT_0065d424;
        FUN_00600c40(this,pvVar5,&uStack_4c);
        pvVar5 = (void *)FUN_00604500((void *)(*(int *)((int)this + 0x160) + 0x6c),0x7a);
        uStack_46 = DAT_0065d424;
        uStack_48 = 0x8011;
        FUN_00600c40(this,pvVar5,&uStack_48);
        pvVar5 = (void *)FUN_00604500((void *)(*(int *)((int)this + 0x160) + 0x6c),0x7d);
        uStack_44 = 0x8012;
        uStack_42 = DAT_0065d424;
        FUN_00600c40(this,pvVar5,&uStack_44);
        pvVar5 = (void *)FUN_00604500((void *)(*(int *)((int)this + 0x160) + 0x6c),0x7e);
        uStack_3e = DAT_0065d424;
        uStack_40 = 0x8013;
        FUN_00600c40(this,pvVar5,&uStack_40);
        pvVar5 = (void *)FUN_00604500((void *)(*(int *)((int)this + 0x160) + 0x6c),0x7c);
        uStack_3c = 0x8014;
        uStack_3a = DAT_0065d424;
        FUN_00600c40(this,pvVar5,&uStack_3c);
        pvVar5 = (void *)FUN_00604500((void *)(*(int *)((int)this + 0x160) + 0x6c),0x7f);
        uStack_36 = DAT_0065d424;
        uStack_38 = 0x8016;
        FUN_00600c40(this,pvVar5,&uStack_38);
        pvVar5 = (void *)FUN_00604500((void *)(*(int *)((int)this + 0x160) + 0x6c),0x81);
        if (pvVar5 != (void *)0x0) {
          uStack_34 = 0x8015;
          uStack_32 = DAT_0065d424;
          FUN_00600c40(this,pvVar5,&uStack_34);
        }
        piVar3 = (int *)FUN_00607e90(this,0x65);
        if (piVar3 != (int *)0x0) {
          uStack_30 = 0x8020;
          uStack_2e = DAT_0065d424;
          FUN_00600c40(this,piVar3,&uStack_30);
          if (*(int *)(*(int *)((int)this + 0x114) + 0x9c) == 1) {
            uVar4 = 0x2a76;
            uVar7 = 0x2a77;
            uVar6 = 0x2ad1;
          }
          else {
            uVar4 = 0x29f2;
            uVar7 = 0x29f3;
            uVar6 = 0x2ad2;
          }
          FUN_00603150(piVar3,0,uVar4);
          FUN_00603150(piVar3,1,uVar7);
          FUN_00603150(piVar3,2,uVar6);
          (**(code **)(*piVar3 + 4))(5);
        }
        FUN_00468ab0(this,1);
        SetRect(&tStack_1c,0xf,0x14,0x19a,0x3c);
        InvalidateRect(*(HWND *)((int)this + 0x18),&tStack_1c,0);
        FUN_0060a280(*(void **)((int)this + 0x168));
        iVar2 = *(int *)((int)this + 0x11c);
        *(undefined4 *)((int)this + 0x11c) = 0;
        FUN_004697b0(this,iVar2);
        SetFocus(*(HWND *)(*(int *)((int)this + 0x168) + 0x18));
        uVar8 = 1;
      }
      else if (iVar2 == 2) {
        FUN_0060a790(local_2c,1);
        local_4 = 0;
        FUN_00609410(*(void **)((int)this + 0x168),local_2c);
        if (local_24 == 0) {
          *(undefined4 *)((int)this + 0x118) = 0;
          (**(code **)(*(int *)this + 0x48))(0x65,0,0);
          *(int *)((int)this + 0x118) = iVar1;
          local_4 = 0xffffffff;
          FUN_0060a810(local_2c);
        }
        else {
          uVar8 = FUN_0060a860(*(void **)((int)this + 0x16c),*(int *)(local_24 + 0xc));
          *(undefined4 *)((int)this + 0x170) = uVar8;
          piVar3 = (int *)FUN_00607e90(this,0x65);
          if (piVar3 != (int *)0x0) {
            local_50 = 0x8019;
            local_4e = DAT_0065d424;
            FUN_00600c40(this,piVar3,&local_50);
            if (*(int *)(*(int *)((int)this + 0x114) + 0x9c) == 1) {
              uVar4 = 0x2884;
              uVar6 = 0x2885;
            }
            else {
              uVar4 = 0x288a;
              uVar6 = 0x288b;
            }
            FUN_00603150(piVar3,0,uVar4);
            FUN_00603150(piVar3,1,uVar6);
            (**(code **)(*piVar3 + 4))(5);
          }
          FUN_006075e0(this,0x34);
          pvVar5 = (void *)((int)this + 0x6c);
          piVar3 = (int *)FUN_00604500(pvVar5,0x91);
          (**(code **)(*piVar3 + 8))();
          piVar3 = (int *)FUN_00604500(pvVar5,0x90);
          (**(code **)(*piVar3 + 8))();
          piVar3 = (int *)FUN_00604500(pvVar5,0x96);
          (**(code **)(*piVar3 + 4))(5);
          piVar3 = (int *)FUN_00604500(pvVar5,0x9a);
          if (piVar3 != (int *)0x0) {
            (**(code **)(*piVar3 + 4))(5);
          }
          piVar3 = (int *)FUN_00604500(pvVar5,0x98);
          (**(code **)(*piVar3 + 8))();
          piVar3 = (int *)FUN_00604500(pvVar5,0x99);
          (**(code **)(*piVar3 + 8))();
          piVar3 = (int *)FUN_00604500(pvVar5,0x97);
          (**(code **)(*piVar3 + 8))();
          FUN_0041fb50(*(void **)((int)this + 0x1dc),0);
          iVar2 = *(int *)((int)this + 0x1dc);
          *(undefined4 *)(iVar2 + 0x130) = 0x140;
          *(undefined4 *)(iVar2 + 300) = 0;
          (**(code **)(**(int **)((int)this + 0x1dc) + 4))(5);
          (**(code **)(**(int **)((int)this + 0x168) + 8))();
          (**(code **)(**(int **)((int)this + 0x160) + 8))();
          FUN_00469de0(this,*(int *)((int)this + 0x170));
          piVar3 = (int *)FUN_00607e90(this,0x65);
          (**(code **)(*piVar3 + 4))(5);
          SetFocus(*(HWND *)(*(int *)((int)this + 0x1dc) + 0x18));
          uVar8 = 1;
          local_4 = 0xffffffff;
          FUN_0060a810(local_2c);
        }
      }
      else if (iVar2 == 3) {
        SetWindowPos(*(HWND *)(*(int *)((int)this + 0x1dc) + 0x18),(HWND)0x0,0,0,0x140,
                     *(int *)(*(int *)((int)this + 0x1dc) + 0x34),6);
        FUN_006075e0(this,0x34);
        FUN_0046a320(this,0,0x411);
        piVar3 = (int *)FUN_00607e90(this,0x65);
        if (piVar3 != (int *)0x0) {
          param_1 = CONCAT22(DAT_0065d424,0x8019);
          FUN_00600c40(this,piVar3,&param_1);
          if (*(int *)(*(int *)((int)this + 0x114) + 0x9c) == 1) {
            uVar4 = 0x2884;
            uVar6 = 0x2885;
          }
          else {
            uVar4 = 0x288a;
            uVar6 = 0x288b;
          }
          FUN_00603150(piVar3,0,uVar4);
          FUN_00603150(piVar3,1,uVar6);
          (**(code **)(*piVar3 + 4))(5);
        }
        pvVar5 = (void *)((int)this + 0x6c);
        piVar3 = (int *)FUN_00604500(pvVar5,0x91);
        (**(code **)(*piVar3 + 8))();
        piVar3 = (int *)FUN_00604500(pvVar5,0x90);
        (**(code **)(*piVar3 + 8))();
        piVar3 = (int *)FUN_00604500(pvVar5,0x96);
        (**(code **)(*piVar3 + 8))();
        piVar3 = (int *)FUN_00604500(pvVar5,0x9a);
        (**(code **)(*piVar3 + 8))();
        piVar3 = (int *)FUN_00604500(pvVar5,0x98);
        if (*(int *)(*(int *)((int)this + 0x114) + 0x9c) == 1) {
          FUN_00603150(piVar3,0,0x2ab6);
          uVar4 = 0x2ab7;
        }
        else {
          FUN_00603150(piVar3,0,0x2ab8);
          uVar4 = 0x2ab9;
        }
        FUN_00603150(piVar3,1,uVar4);
        (**(code **)(*piVar3 + 4))(5);
        piVar3 = (int *)FUN_00604500(pvVar5,0x99);
        FUN_00603150(piVar3,0,0x2aba);
        FUN_00603150(piVar3,1,0x2abb);
        (**(code **)(*piVar3 + 4))(5);
        piVar3 = (int *)FUN_00604500(pvVar5,0x97);
        (**(code **)(*piVar3 + 8))();
        FUN_005f2f50((undefined4 *)&uStack_34);
        local_c = (void *)0x1;
        if ((*(byte *)((int)*(void **)((int)this + 0x1dc) + 0x138) & 1) == 0) {
          FUN_0041fb50(*(void **)((int)this + 0x1dc),1);
        }
        iVar2 = *(int *)((int)this + 0x1dc);
        *(undefined4 *)(iVar2 + 0x130) = 0x140;
        *(undefined4 *)(iVar2 + 300) = 0;
        pvVar5 = *(void **)((int)this + 0x1dc);
        FUN_005f3090((void *)((int)pvVar5 + 0xa0),(int)&uStack_34);
        *(undefined4 *)((int)pvVar5 + 0x13c) = 0;
        FUN_0041fc30(pvVar5);
        (**(code **)(**(int **)((int)this + 0x1dc) + 4))(5);
        SetFocus(*(HWND *)(*(int *)((int)this + 0x1dc) + 0x18));
        (**(code **)(**(int **)((int)this + 0x168) + 8))();
        (**(code **)(**(int **)((int)this + 0x160) + 8))();
        SetFocus(*(HWND *)(*(int *)((int)this + 0x1dc) + 0x18));
        uVar8 = 1;
        local_4 = 0xffffffff;
        FUN_005f2ff0(local_2c);
      }
      else {
        uVar8 = 0;
      }
    }
  }
  ExceptionList = local_c;
  return uVar8;
}

